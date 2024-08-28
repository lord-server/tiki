use tiki_macros::Serialize;

use crate::Error;

#[tiki_macros::packet]
#[derive(Debug)]
pub enum Serverbound {
    #[id = 0x00]
    Hello(Hello),

    #[id = 0x02]
    Init(Init),

    #[id = 0x11]
    Init2(Init2),

    #[id = 0x17]
    ModChannelJoin(ModChannelJoin),

    #[id = 0x18]
    ModChannelLeave(ModChannelLeave),

    #[id = 0x19]
    ModChannelMsg(ModChannelMsg),

    #[id = 0x23]
    PlayerPos(PlayerPos),

    #[id = 0x24]
    GotBlocks(GotBlocks),

    #[id = 0x25]
    DeletedBlocks(DeletedBlocks),

    #[id = 0x31]
    InventoryAction(InventoryAction),

    #[id = 0x32]
    ChatMessage(ChatMessage),

    #[id = 0x35]
    Damage(Damage),

    #[id = 0x37]
    PlayerItem(PlayerItem),

    #[id = 0x38]
    Respawn(Respawn),

    #[id = 0x39]
    Interact(Interact),

    #[id = 0x3A]
    RemovedSounds(RemovedSounds),

    #[id = 0x3B]
    NodeMetaFields(NodeMetaFields),

    #[id = 0x3C]
    InventoryFields(InventoryFields),

    #[id = 0x40]
    RequestMedia(RequestMedia),

    #[id = 0x41]
    HaveMedia(HaveMedia),

    #[id = 0x43]
    ClientReady(ClientReady),

    #[id = 0x50]
    FirstSrp(FirstSrp),

    #[id = 0x51]
    SrpBytesA(SrpBytesA),

    #[id = 0x52]
    SrpBytesM(SrpBytesM),

    #[id = 0x53]
    UpdateClientInfo(UpdateClientInfo),
}

#[derive(Serialize, Debug)]
pub struct Hello {}

#[derive(Serialize, Debug)]
pub struct Init {
    pub client_max_serialization_ver: u8,
    pub supp_compr_modes: u16,
    pub min_net_proto_version: u16,
    pub max_net_proto_version: u16,
    pub player_name: String,
}

#[derive(Serialize, Debug)]
pub struct Init2 {
    pub lang: String,
}

#[derive(Serialize, Debug)]
pub struct ModChannelJoin {}

#[derive(Serialize, Debug)]
pub struct ModChannelLeave {}

#[derive(Serialize, Debug)]
pub struct ModChannelMsg {}

#[derive(Serialize, Debug)]
pub struct PlayerPos {}

#[derive(Serialize, Debug)]
pub struct GotBlocks {}

#[derive(Serialize, Debug)]
pub struct DeletedBlocks {}

#[derive(Serialize, Debug)]
pub struct InventoryAction {}

#[derive(Serialize, Debug)]
pub struct ChatMessage {}

#[derive(Serialize, Debug)]
pub struct Damage {}

#[derive(Serialize, Debug)]
pub struct PlayerItem {}

#[derive(Serialize, Debug)]
pub struct Respawn {}

#[derive(Serialize, Debug)]
pub struct Interact {}

#[derive(Serialize, Debug)]
pub struct RemovedSounds {}

#[derive(Serialize, Debug)]
pub struct NodeMetaFields {}

#[derive(Serialize, Debug)]
pub struct InventoryFields {}

#[derive(Serialize, Debug)]
pub struct RequestMedia {}

#[derive(Serialize, Debug)]
pub struct HaveMedia {}

#[derive(Serialize, Debug)]
pub struct ClientReady {}

#[derive(Serialize, Debug)]
pub struct FirstSrp {}

#[derive(Serialize, Debug)]
pub struct SrpBytesA {}

#[derive(Serialize, Debug)]
pub struct SrpBytesM {}

#[derive(Serialize, Debug)]
pub struct UpdateClientInfo {}
