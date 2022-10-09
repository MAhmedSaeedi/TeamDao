use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

//Creating Team and defining structure of TeamDAO
#[program]
pub mod team {
    use super::*;

    //creating a team
    pub fn create_team(ctx: Context<CreateTeam>,winners:u8) -> Result<()> {
        require!(winners > 0,TeamError::WinnerCountNotAllowed);
        let team = &mut ctx.accounts.team_data;
        team.members = 0; 
        team.initiator = ctx.accounts.signer.key();
        team.winners_num = winners;
        Ok(())
    }

    //changing owner of the team
    pub fn change_team_initiator(ctx:Context<ChangeInitiator>) -> Result<()>{
        let team = &mut ctx.accounts.team_data;

        team.initiator = ctx.accounts.signer.key();
        Ok(())
    }


    //apply for the team
    pub fn apply(ctx: Context<Apply>) -> Result<()> {
        let team = &mut ctx.accounts.team_data;
        team.members += 1;
        ctx.accounts.member_identity.id = team.members;
        ctx.accounts.member_identity.pubkey = ctx.accounts.signer.key();
        Ok(())
    }

    //registeration as team member
    pub fn register(ctx: Context<Register>) -> Result<()> {
        let member = &mut ctx.accounts.member_data;
        member.votes = 0;
        member.pubkey = ctx.accounts.signer.key();
        member.id = ctx.accounts.member_identity.id;
        Ok(())
    }

    //remove from team
    pub fn remove(ctx: Context<Remove>) -> Result<()> {
        let member = &mut ctx.accounts.member_data;

        member.votes = 0;
        member.pubkey = ctx.accounts.signer.key();
        member.id = 0;
    
        Ok(())
    }

    //Vote for the team
    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        let team = &mut ctx.accounts.team_data;
    
    
        let member = &mut ctx.accounts.member_data;
        let my_vote = &mut ctx.accounts.my_vote;
    
        member.votes += 1;
        my_vote.id = member.id;
    
        team.record_vote(member.id,member.votes);
        
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(winners:u8)]
pub struct CreateTeam<'info> {
    #[account(
        init,
        payer=signer,
        space= 8 + 8 + 2 + 32 + 1 + 2 * (4 + winners as usize * 8)
    )]
    pub team_data: Account<'info,TeamData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct ChangeInitiator<'info> {
    #[account(
        init,
        payer=signer,
        space= 8 + 8 + 2 + 32 + 1 + 2 * (4  as usize * 8)
    )]
    pub team_data: Account<'info,TeamData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct Apply<'info> {
    #[account(
        init,
        payer=signer,
        space=8+8+32,
        seeds=[
            b"team",
            signer.key().as_ref(),
            team_data.key().as_ref()
        ],
        bump
    )]
    pub member_identity: Account<'info,MemberIdentity>,
    #[account(mut)]
    pub team_data: Account<'info,TeamData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info,System>
}


#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(
        init,
        payer=signer,
        space=8+8,
        seeds=[
            b"voter",
            signer.key().as_ref(),
            team_data.key().as_ref()
        ],
        bump
    )]
    pub my_vote: Account<'info,MyVote>,
    #[account(mut)]
    pub member_data: Account<'info,MemberData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub team_data: Account<'info,TeamData>,
    pub system_program: Program<'info,System>
}


#[derive(Accounts)]
pub struct Register<'info> {
    #[account(
        init,
        payer=signer,
        space=8+8+8+32,
        seeds=[
            &(member_identity.id).to_be_bytes(),
            team_data.key().as_ref()
        ],
        bump
    )]
    pub member_data: Account<'info,MemberData>,
    pub team_data: Account<'info,TeamData>,
    pub member_identity: Account<'info,MemberIdentity>,
    #[account(mut,address=member_identity.pubkey @ TeamError::WrongPublicKey)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct Remove<'info> {
    #[account(
        init,
        payer=signer,
        space=8+8+8+32,
        seeds=[
            &(member_identity.id).to_be_bytes(),
            team_data.key().as_ref()
        ],
        bump
    )]
    pub member_data: Account<'info,MemberData>,
    pub team_data: Account<'info,TeamData>,
    pub member_identity: Account<'info,MemberIdentity>,
    #[account(mut,address=member_identity.pubkey @ TeamError::WrongPublicKey)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info,System>
}




#[account]
pub struct TeamData {
    pub members: u64,
    pub initiator: Pubkey,
    pub winners_num: u8,
    pub winners_id: Vec<u64>,
    pub winners_votes: Vec<u64>,
}

#[account]
pub struct MemberData {
    pub votes: u64,
    pub id: u64,
    pub pubkey: Pubkey,
}

#[account]
pub struct MemberIdentity {
    pub id: u64,
    pub pubkey: Pubkey,
}

#[account]
pub struct MyVote {
    pub id: u64,
}


impl TeamData {
    pub fn close_application(&mut self) -> Result<()> {   
        if self.members <= self.winners_num as u64 {
            for i in 1..self.members + 1 {
                self.winners_id.push(i);
            }
        }
        Ok(())
    }

    pub fn record_vote(&mut self,id: u64,votes: u64) {
        if !self.winners_id.contains(&id) {
            if self.winners_id.len() < self.winners_num as usize {
                self.winners_id.push(id);
                self.winners_votes.push(votes);            
            } else {
                let current_last_winner = (self.winners_num - 1) as usize;

                if votes > self.winners_votes[current_last_winner] {
                    self.winners_id[current_last_winner] = id;
                    self.winners_votes[current_last_winner] = votes;
                } else {
                    return;
                }
            }
        } else {
            let index = self.winners_id.iter().position(|&r| r == id).unwrap();
            self.winners_votes[index] += 1;
        }
        
        //sorting votes in descending order if winners' votes are changed
        let mut j = self.winners_id.iter().position(|&r| r == id).unwrap();

        while j > 0 && self.winners_votes[j] > self.winners_votes[j-1] {

            let vote_holder = self.winners_votes[j-1];
            let id_holder = self.winners_id[j-1];

            self.winners_votes[j-1] = self.winners_votes[j];
            self.winners_votes[j] = vote_holder;

            self.winners_id[j-1] = self.winners_id[j];
            self.winners_id[j] = id_holder;

            j -= 1;
        }
    }
    
}


#[error_code]
pub enum TeamError {
    WinnerCountNotAllowed,
    WrongPublicKey
}



