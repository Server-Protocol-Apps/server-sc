## Entidades

- **Repo**: Repositorios propuestos y aprobados
- **Admin**: Clave publica del signer con privilegios de administrador y clave publica descomprimida de ethereum del backend firmador de cupones
- **Subscription**: Quien esta suscrito a cierto repo, cuantos rewards ha reclamado y cuando lo hizo la última vez
- **Vote**: Quien voto a cierto repo y cuando

## Instrucciones

En Solana, se interacciona con los contratos a través de instrucciones, cada instrucción tiene un contexto y un payload opcional,
las intrucciones estan dentro de **src/instructions** y dependen de los tipos definidos en **src/state**

Solana procesa las transacciones en paralelo y, para eso, necesita saber que datos van a ser modificados

Através del contexto es como dejamos saber a Solana que datos vamos a tocar

Declaramos los datos que vamos a leer, añadir o modificar junto al tipo que tendrán

En _add_repo.rs_, el contexto es el siguiente:

```
#[derive(Accounts)]
#[instruction(payload: AddRepoPayload)]
pub struct AddRepo<'info> {
   #[account(
       init,
       seeds = [b"repo", ...],
       bump,
       payer = publisher,
       space = Repo::size(...)
   )]
   pub repo: Account<'info, Repo>,
   #[account(
       seeds = [b"ADMIN"],
       bump,
   )]
   pub admin: Account<'info, Admin>,
   #[account(mut)]
   pub publisher: Signer<'info>,
   pub system_program: Program<'info, System>,
}
```

El contexto tiene 4 atributos, **repo**, **admin**, **publisher** y **system_program** y dice algo como esto:

- Dame los datos del **admin** de este smart contract
- Escribe en el smart contract un nuevo **repo**, sus valores estan en **payload**
- Los gastos corren a cuenta de **publisher**
- Y para poder guardar los datos del repositorio, vas a tener que usar **system_program**

## Accounts

Todo son cuentas en Solana, _programs_, _pdas_, _signers_, _atas_, _mint_, etc

### PDAs

PDAs son el equivalente a los datos que guardariamos en una base de datos tradicional en cualquier otra aplicación

No viven en la curva eliptica sino que se derivan através del ID del programa y una seed que definimos en el contrato, podemos pensar
sobre estas seeds como la public key en cualquier otra base de datos

**repo** y **admin** en el ejemplo anterior son PDAs

### Mint

Mint es la cuenta que guarda información sobre la emision de un token, como quien es el owner, authority, supply, decimals, nombre, etc

### ATAs

ATAs o, associated token account, represetan el balance de un signer y la transferencia de tokens se hacen através de ellas, se derivan usando un mint y un signer

## CPI & Token Program

CPI son las siglas de Cross Program Invocation, aunque creemos un token llamando a Server, no es Server quien lo crea sino que delega la tarea al token_program, otro programa que vive en Solana y que se encarga de todas las cosas relacionadas con los tokens, inicializarlos, mintear, quemar, transferir, etc

## Cupones

Algunas instrucciones necesitan que el payload esté verificado por una entidad reconocida y fiable, que en este caso, es el backend definido en **server-app**

Para validar el payload y confirmar que nuestra entidad fiable lo ha firmado, usamos cupones

Usando los parametros de entrada que se enviarán al programa, el backend genera un cupón y lo firma, esta firma se enviará junto a los parametros al programa donde este construirá un cupón con los parametros y lo usa junto a la firma para recuperar la dirección de quien firmo el cupón original

`secp256k1_recover(&hash, self.recovery_id, signature).unwrap()` (hash es el cupón)

Si el address recuperado es el mismo que el address que tenemos guardado en la entidad Admin, podemos confiar en los parametros recibidos

Para evitar usar el mismo cupón mas de una vez, se envia el timestamp en cada transaccion y se guarda el último timestamp usado en el programa para poder compraralo con el de la siguiente llamada

## Desplegar en local

Para lanzar en local, hace falta tener instalado rust, anchor y solana CLI

#### Configurar solana CLI

`solana config get` debería de mostrar algo así

```
Config File: /Users/kilojulio/.config/solana/cli/config.yml
RPC URL: http://127.0.0.1:8899
WebSocket URL: ws://127.0.0.1:8900/ (computed)
Keypair Path: /Users/kilojulio/.config/solana/id.json
Commitment: confirmed
```

#### Ejecutar el nodo donde desplegar en contrato

`solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so`

#### Desplegar el contrato

`anchor deploy`

#### Inicializar el contrato

`npm run init`

## Desplegar a testnet o mainnet

Los pasos son los mismos, solo hay que cambiar la configuración del solana CLI, asegurarse de que hay SOL suficiente en la cuenta
donde apunta **Kepair Path** y cambiar la url de rpc en `scripts/init.ts`
