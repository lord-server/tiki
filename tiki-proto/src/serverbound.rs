use tiki_macros::Serialize;

use crate::Error;

#[tiki_macros::packet]
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

#[derive(Serialize)]
pub struct Hello {}

#[derive(Serialize)]
pub struct Init {}

#[derive(Serialize)]
pub struct Init2 {}

#[derive(Serialize)]
pub struct ModChannelJoin {}

#[derive(Serialize)]
pub struct ModChannelLeave {}

#[derive(Serialize)]
pub struct ModChannelMsg {}

#[derive(Serialize)]
pub struct PlayerPos {}

#[derive(Serialize)]
pub struct GotBlocks {}

#[derive(Serialize)]
pub struct DeletedBlocks {}

#[derive(Serialize)]
pub struct InventoryAction {}

#[derive(Serialize)]
pub struct ChatMessage {}

#[derive(Serialize)]
pub struct Damage {}

#[derive(Serialize)]
pub struct PlayerItem {}

#[derive(Serialize)]
pub struct Respawn {}

#[derive(Serialize)]
pub struct Interact {}

#[derive(Serialize)]
pub struct RemovedSounds {}

#[derive(Serialize)]
pub struct NodeMetaFields {}

#[derive(Serialize)]
pub struct InventoryFields {}

#[derive(Serialize)]
pub struct RequestMedia {}

#[derive(Serialize)]
pub struct HaveMedia {}

#[derive(Serialize)]
pub struct ClientReady {}

#[derive(Serialize)]
pub struct FirstSrp {}

#[derive(Serialize)]
pub struct SrpBytesA {}

#[derive(Serialize)]
pub struct SrpBytesM {}

#[derive(Serialize)]
pub struct UpdateClientInfo {}
