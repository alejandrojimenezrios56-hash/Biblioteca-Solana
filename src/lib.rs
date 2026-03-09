use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod renta_autos {
    use super::*;

    //////////////////////////// Crear Agencia /////////////////////////////////////
    pub fn crear_agencia(context: Context<NuevaAgencia>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let autos: Vec<Auto> = Vec::new();

        context.accounts.agencia.set_inner(AgenciaAutos {
            owner: owner_id,
            nombre,
            autos,
        });

        Ok(())
    }

    //////////////////////////// Agregar Auto /////////////////////////////////////
    pub fn agregar_auto(context: Context<NuevoAuto>, nombre: String, precio: u16) -> Result<()> {

        require!(
            context.accounts.agencia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let auto = Auto {
            nombre,
            precio,
            disponible: true,
        };

        context.accounts.agencia.autos.push(auto);

        Ok(())
    }

    //////////////////////////// Eliminar Auto /////////////////////////////////////
    pub fn eliminar_auto(context: Context<NuevoAuto>, nombre: String) -> Result<()> {

        require!(
            context.accounts.agencia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let autos = &mut context.accounts.agencia.autos;

        for i in 0..autos.len() {
            if autos[i].nombre == nombre {
                autos.remove(i);
                msg!("Auto {} eliminado!", nombre);
                return Ok(());
            }
        }

        Err(Errores::AutoNoExiste.into())
    }

    //////////////////////////// Ver Autos /////////////////////////////////////
    pub fn ver_autos(context: Context<NuevoAuto>) -> Result<()> {

        require!(
            context.accounts.agencia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Lista de autos disponibles: {:#?}", context.accounts.agencia.autos);

        Ok(())
    }

    //////////////////////////// Cambiar Estado /////////////////////////////////////
    pub fn alternar_estado(context: Context<NuevoAuto>, nombre: String) -> Result<()> {

        require!(
            context.accounts.agencia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let autos = &mut context.accounts.agencia.autos;

        for i in 0..autos.len() {

            let estado = autos[i].disponible;

            if autos[i].nombre == nombre {

                let nuevo_estado = !estado;

                autos[i].disponible = nuevo_estado;

                msg!(
                    "El auto: {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(Errores::AutoNoExiste.into())
    }
}

#[error_code]
pub enum Errores {

    #[msg("Error, no eres el propietario de la agencia")]
    NoEresElOwner,

    #[msg("Error, el auto no existe")]
    AutoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct AgenciaAutos {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    autos: Vec<Auto>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Auto {

    #[max_len(60)]
    nombre: String,

    precio: u16,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaAgencia<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = AgenciaAutos::INIT_SPACE + 8,
        seeds = [b"agencia", owner.key().as_ref()],
        bump
    )]
    pub agencia: Account<'info, AgenciaAutos>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoAuto<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub agencia: Account<'info, AgenciaAutos>,
}
