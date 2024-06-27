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

    private PublicKey rng_programId = new PublicKey("9uSwASSU59XvUS8d1UeU8EwrEzMGFdXZvQ4JSEAfcS7k");
    private PublicKey coin_flip_programId = new PublicKey("5uNCDQwxG8dgdFsAYMzb6DS442bLbRp85P2dAn15rt4d");
    private PublicKey current_feed_account;
    private byte bump;


    public ulong decision;

    public async void Play()
    {


        byte[] seed1 = Encoding.UTF8.GetBytes("c");
        byte[] seed2 = new byte[] { 1 };

       PublicKey.TryFindProgramAddress(new List<byte[]>{seed1,seed2},rng_programId, out current_feed_account, out bump);


       var result = await  rpcClient.GetAccountInfoAsync(current_feed_account);
       var data = result.Result.Value.Data;

       byte[] encoded_data = Convert.FromBase64String(data.First());


        var account1 = Deserialization.GetPubKey(encoded_data, 17);
        var account2 = Deserialization.GetPubKey(encoded_data, 49);
        var account3 = Deserialization.GetPubKey(encoded_data, 81);
        var fallback_account = Deserialization.GetPubKey(encoded_data, 113);

        Account temp =  new Account();

        byte[] buffer = new byte[8]; // Ensure the buffer is large enough for a u64 (8 bytes)

        

        Serialization.WriteU64(buffer, decision, 0);
        


        var ix = new TransactionInstruction
           {
                ProgramId = coin_flip_programId,
                Keys = new List<AccountMeta>{AccountMeta.Writable(player.PublicKey,isSigner:true),
                        AccountMeta.ReadOnly(account1,isSigner:false),
                        AccountMeta.ReadOnly(account2,isSigner:false),
                        AccountMeta.ReadOnly(account3,isSigner:false),
                        AccountMeta.ReadOnly(fallback_account,isSigner:false),
                        AccountMeta.Writable(current_feed_account,isSigner:false),
                        AccountMeta.Writable(temp.PublicKey,isSigner:true),
                        AccountMeta.ReadOnly(rng_programId,isSigner:false),
                        AccountMeta.ReadOnly(SystemProgram.ProgramIdKey,isSigner:false),
                 },
                Data = buffer
           };
        

       var tx = new TransactionBuilder()
            .SetRecentBlockHash((await rpcClient.GetLatestBlockHashAsync()).Result.Value.Blockhash)
            .SetFeePayer(player.PublicKey)
            .AddInstruction(ix)
            .Build(new List<Account> { player,temp });



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

