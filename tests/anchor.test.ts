// No imports needed: web3, anchor, pg and more are globally available

describe("Renta de Autos Test", () => {

  it("Crear agencia", async () => {

    // PDA de la agencia
    const [agenciaPda] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("agencia"),
        pg.wallet.publicKey.toBuffer()
      ],
      pg.program.programId
    );

    const nombreAgencia = "Autos Omar";

    // Ejecutar instrucción del programa
    const txHash = await pg.program.methods
      .crearAgencia(nombreAgencia)
      .accounts({
        owner: pg.wallet.publicKey,
        agencia: agenciaPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirmar transacción
    await pg.connection.confirmTransaction(txHash);

    // Obtener datos guardados en blockchain
    const agencia = await pg.program.account.agenciaAutos.fetch(agenciaPda);

    console.log("Datos de la agencia:", agencia);

    // Validación
    assert(agencia.nombre === nombreAgencia);

  });

});
