#[derive(Debug)]
pub struct ParseError;

#[non_exhaustive]
#[derive(Debug)]
pub enum Pkt {
    ServerChangeInfo,
    SyncNearEntities = 0x00000006,  
    SyncContainerData = 0x00000015,
    SyncServerTime = 0x0000002b,
    SyncToMeDeltaInfo = 0x0000002e,
    SyncNearDeltaInfo = 0x0000002d,
}

impl TryFrom<u32> for Pkt {
    type Error = ParseError;

    fn try_from(pkt: u32) -> Result<Self, Self::Error> {
        match pkt {
            0x00000006 => Ok(Pkt::SyncNearEntities),
            0x00000015 => Ok(Pkt::SyncContainerData),
            0x0000002b => Ok(Pkt::SyncServerTime),
            0x0000002e => Ok(Pkt::SyncToMeDeltaInfo),
            0x0000002d => Ok(Pkt::SyncNearDeltaInfo),
            _ => Err(ParseError),
        }
    }
}

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug)]
pub enum FragmentType {
    None = 0,
    Call = 1,
    Notify = 2,
    Return = 3,
    Echo = 4,
    FrameUp = 5,
    FrameDown = 6,
}

impl From<u16> for FragmentType {
    fn from(fragment_type: u16) -> Self {
        match fragment_type {
            0 => FragmentType::None,
            1 => FragmentType::Call,
            2 => FragmentType::Notify,
            3 => FragmentType::Return,
            4 => FragmentType::Echo,
            5 => FragmentType::FrameUp,
            6 => FragmentType::FrameDown,
            _ => FragmentType::None,
        }
    }
}

/*
Pkt::DeathNotify
Pkt::IdentityGaugeChangeNotify
Pkt::IdentityStanceChangeNotify
Pkt::InitEnv
Pkt::InitPC
Pkt::InitItem
Pkt::MigrationExecute
Pkt::NewPC
Pkt::NewVehicle
Pkt::NewNpc
Pkt::NewNpcSummon
Pkt::NewProjectile
Pkt::NewTrap
Pkt::ParalyzationStateNotify
Pkt::RaidBegin
Pkt::RaidBossKillNotify
Pkt::RaidResult
Pkt::RemoveObject
Pkt::SkillCastNotify
Pkt::SkillCooldownNotify
Pkt::SkillStartNotify
Pkt::SkillStageNotify
Pkt::SkillDamageAbnormalMoveNotify
Pkt::SkillDamageNotify
Pkt::PartyInfo
Pkt::PartyLeaveResult
Pkt::PartyStatusEffectAddNotify
Pkt::PartyStatusEffectRemoveNotify
Pkt::PartyStatusEffectResultNotify
Pkt::StatusEffectAddNotify
Pkt::StatusEffectDurationNotify
Pkt::StatusEffectRemoveNotify
Pkt::TriggerBossBattleStatus
Pkt::TriggerStartNotify
Pkt::ZoneMemberLoadStatusNotify
Pkt::ZoneObjectUnpublishNotify
Pkt::StatusEffectSyncDataNotify
Pkt::TroopMemberUpdateMinNotify
Pkt::NewTransit
 */
