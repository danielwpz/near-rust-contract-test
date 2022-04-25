import { Gas, NEAR, Workspace } from 'near-workspaces-ava';

const workspace = Workspace.init(async ({ root }) => {
    const owner = await root.createAccount('owner');
    const alice = await root.createAccount('alice');
    const bob = await root.createAccount('bob');
    const carol = await root.createAccount('carol');

    const ft = await root.createAndDeploy(
        'ft',
        'res/mock_ft.wasm',
        {
            method: 'new',
            args: {}
        }
    );

    const contract = await root.createAndDeploy(
        'lottery',
        'res/lottery.wasm',
        {
            method: 'new',
            args: {
                owner_id: owner.accountId,
                reward_token_id: ft.accountId,
                ticket_price: NEAR.parse('1').toString(10)
            }
        }
    );

    // mint ft for lottery
    await alice.call(
        ft,
        'mint',
        {
            account_id: contract.accountId,
            amount: NEAR.parse('5')
        }
    );

    return {
        owner,
        contract,
        ft,
        alice,
        bob,
        carol,
    }
});

workspace.test('Buy ticket', async (test, { contract, alice,}) => {
    await alice.call(
        contract,
        'buy_ticket',
        {},
        {
            attachedDeposit: NEAR.parse('1')
        }
    );

    const players: string[] = await contract.view('get_players');
    test.is(players.length, 1);
    test.is(players[0], alice.accountId);
});

workspace.test('draw and claim', async (test, { contract, ft, owner, alice, bob }) => {
    await alice.call(
        contract,
        'buy_ticket',
        {},
        {
            attachedDeposit: NEAR.parse('1')
        }
    );
 
    await bob.call(
        contract,
        'buy_ticket',
        {},
        {
            attachedDeposit: NEAR.parse('1')
        }
    );

    await owner.call(
        contract,
        'draw',
        {
            n: 1
        }
    );

    const winners: string[] = await contract.view('get_winners');
    const winner = winners[0] === alice.accountId ? alice : bob;

    // register
    await winner.call(
        ft,
        'storage_deposit',
        {
            account_id: winner.accountId
        },
        {
            attachedDeposit: NEAR.parse('0.1')
        }
    );

    await winner.call(
        contract,
        'claim',
        {},
        {
            gas: Gas.parse('100 Tgas')
        }
    );

    test.is(
        await ft.view('ft_balance_of', { account_id: winner.accountId }),
        NEAR.parse('1').toString(10)
    );

    // cannot claim twice
    let failed = false;
    try {
        await winner.call(
            contract,
            'claim',
            {},
            {
                gas: Gas.parse('100 Tgas')
            }
        );
    } catch (err) {
        failed = true;
    }

    test.assert(failed);
});
