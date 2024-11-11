using System.Collections.Generic;
using Solana.Unity.SDK;
using Solana.Unity.Wallet;
using UnityEngine;
using Solana.Unity.Programs;
using Solana.Unity.Rpc;
using Solana.Unity.Rpc.Models;
using System;
using System.Linq;
using System.Text;
using Solana.Unity.Programs.Utilities;
using Solana.Unity.Rpc.Builders;
using Solana.Unity.Rpc.Core.Http;




public class RNG_Client : MonoBehaviour
{
   private static readonly IRpcClient rpcClient = ClientFactory.GetClient(Cluster.DevNet);

    Account player;

    private PublicKey rng_programId = new PublicKey("FEED1qspts3SRuoEyG29NMNpsTKX8yG9NGMinNC4GeYB");
    private PublicKey coin_flip_programId = new PublicKey("5uNCDQwxG8dgdFsAYMzb6DS442bLbRp85P2dAn15rt4d");
    private PublicKey entropy_account = new PublicKey("CTyyJKQHo6JhtVYBaXcota9NozebV3vHF872S8ag2TUS");
    private PublicKey fee_account = new PublicKey("WjtcArL5m5peH8ZmAdTtyFF9qjyNxjQ2qp4Gz1YEQdy");



    public ulong decision;

    public async void Play()
    {


       /*
        Entropy account and fee accounts are PDAs. You can also derive them as below

        private PublicKey entropy_account;
        private PublicKey fee_account;
        private byte entropy_account_bump;
        private byte fee_account_bump;

        byte[] entropy_account_seed = Encoding.UTF8.GetBytes("entropy");
        byte[] fee_account_seed = Encoding.UTF8.GetBytes("f");

        PublicKey.TryFindProgramAddress(new List<byte[]>{entropy_account_seed},rng_programId, out entropy_account, out entropy_account_bump);
        PublicKey.TryFindProgramAddress(new List<byte[]>{fee_account_seed},rng_programId, out fee_account, out fee_account_bump);
      */

        byte[] buffer = new byte[8]; // Ensure the buffer is large enough for a u64 (8 bytes)


        Serialization.WriteU64(buffer, decision, 0);
        


        var ix = new TransactionInstruction
           {
                ProgramId = coin_flip_programId,
                Keys = new List<AccountMeta>{AccountMeta.Writable(player.PublicKey,isSigner:true),
                        AccountMeta.Writable(entropy_account_seed,isSigner:false),
                        AccountMeta.Writable(fee_account,isSigner:false),
                        AccountMeta.ReadOnly(rng_programId,isSigner:false),
                        AccountMeta.ReadOnly(SystemProgram.ProgramIdKey,isSigner:false),
                 },
                Data = buffer
           };
        

       var tx = new TransactionBuilder()
            .SetRecentBlockHash((await rpcClient.GetLatestBlockHashAsync()).Result.Value.Blockhash)
            .SetFeePayer(player.PublicKey)
            .AddInstruction(ix)
            .Build(new List<Account> { player });



        RequestResult<string> firstSig = await rpcClient.SendTransactionAsync(tx);


    }



    private void OnEnable(){
        Web3.OnLogin += OnLogin;
    }


    private void OnDisable(){
        Web3.OnLogin -= OnLogin;
    }

    private void OnLogin(Account account)
    {
        player = account;

    }


}


