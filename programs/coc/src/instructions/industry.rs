use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::states::*;
use crate::utils::events::*;

use anchor_spl::token_2022::{
    burn, Burn, Token2022
};
use anchor_spl::token::Mint as TokenMint;
use anchor_spl::token::TokenAccount as SplTokenAccount; 

#[derive(Accounts)]
pub struct OnboardIndustry<'info> {

    #[account(
        init,
        payer = payer,
        space = 8 + 500,
        seeds = [b"industry", industry_authority.key().as_ref()],
        bump
    )]
    pub industry: Account<'info, Industry>,

    #[account(
        seeds = [b"user_role", b"KYC_AUTHORITY"],
        bump = kyc_authority_role.bump
    )]
    pub kyc_authority_role: Account<'info, UserRole>,


    /// validation : this is the instustry's authority
    pub industry_authority: AccountInfo<'info>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReportEmission<'info>{
    #[account(mut)]
    pub industry : Account <'info , Industry>,

    #[account(mut)]
    pub industry_token_account : Account<'info , SplTokenAccount>,

    pub token_mint : Account<'info , TokenMint> , 

    #[account(mut)]
    pub industry_authority : Signer<'info> , 

    pub token_program : Program<'info , Token2022> // Token 22 program
}


impl<'info> OnboardIndustry <'info> {
    pub fn onboard_industry (
        &mut self , company_name : String , registration_number : String , bond_amount : u64 , bump : u8
    )-> Result<()>{
        // validations 

        // check the caller has the correct role - kYC authority 
        require!(
            self.authority.is_signer,
            ErrorCode::InsufficientPermissions,
        ); 
        // the company details are valid and not empty
        require!(!company_name.trim().is_empty(), ErrorCode::InvalidCompany);
        require!(!registration_number.trim().is_empty(), ErrorCode::InvalidRegistrationNumber); 

        // the bond amount meets the minimum requirement 
        require!(bond_amount > 0 , ErrorCode::InvalidBondAmount);

        require!(
            self.industry_authority.is_signer , ErrorCode::Unauthorized 
        );

        // the industry doesnt already exists 
        require!(
            self.system_program.key() == System::id(), 
            ErrorCode :: InvalidSystemProgram
        );


        let industry = &mut self.industry;
        industry.authority = self.industry_authority.key();
        industry.company_name = company_name;
        industry.registration_number = registration_number;
        industry.bond_amount = bond_amount;
        industry.is_kyc_verified = true;
        industry.is_active = true;
        industry.total_emissions = 0;
        industry.credits_burned = 0;
        industry.compliance_status = ComplianceStatus::Compliant;
        industry.onboarding_date = Clock::get()?.unix_timestamp;
        industry.bump = bump;



        // emit onboarding event 

        emit!(IndustryOnboarded{
            industry : self.industry_authority.key(),
            company_name : industry.company_name.clone(),
            bond_amount ,
            timestamp : industry.onboarding_date,
        }); 

        Ok(())
    }
}


impl <'info> ReportEmission <'info>{
    pub fn report_emissions(
        &mut self ,
        co2_tonnes : u64 ,
        reporting_period : String,
    )-> Result<()>{
        // validate that the industry account is active 
        require!(self.industry.is_active , ErrorCode::IndustryNotActive);


        // validate emission amount is > 0
        require!(co2_tonnes > 0 , ErrorCode::InvalidEmissionAmount);

        // Ensure the token account actually belongs to this industry authority
        require!(self.industry_token_account.owner == self.industry_authority.key() , ErrorCode::InvalidTokenAccountOwner);

        // ensure the token account actually beligs to this industry authority 


        // get the current balance of credits the industry holds 
        let current_balance = self.industry_token_account.amount; 

        // compute how many credits can actually be burn 
        let burn_amount= current_balance.min(co2_tonnes); 
    

        if burn_amount > 0 {

            // perform the actual token burn using token 22 CPI
            burn (
                CpiContext::new (
                    self.token_program.to_account_info(), 
                    Burn {
                        from : self.industry_token_account.to_account_info(), 
                        mint : self.token_mint.to_account_info(), 
                        authority : self.industry_authority.to_account_info()
                    },
                ), 
                burn_amount, 
            )?;
        
        }
        
        // update the state 

        self.industry.credits_burned = self.industry.credits_burned.checked_add(burn_amount).ok_or(ErrorCode::MathOverflow)?;

        self.industry.total_emissions = self.industry.total_emissions.checked_add(co2_tonnes).ok_or(ErrorCode::MathOverflow)?;


        // set complaince based on whether all emission were offset
        self.industry.compliance_status = if burn_amount >= co2_tonnes {
            ComplianceStatus::Compliant
        }
        else {
            ComplianceStatus::NonCompliant
        };

        // emit the event for indexing , analytics
        emit!(EmissionsReported {
            industry : self.industry_authority.key() , 
            co2_tonnes , 
            credits_burned : burn_amount , 
            reporting_period , 
            compliance_status : self.industry.compliance_status.clone(), 
            timestamp : Clock::get()?.unix_timestamp, 
        }); 

        Ok(())
    }
}