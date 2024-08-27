use tiki_macros::Serialize;

use crate::Error;

#[tiki_macros::packet]
#[derive(Debug)]
pub enum Clientbound {
    #[id = 0x02]
    Hello(Hello),

    #[id = 0x03]
    AuthAccept(AuthAccept),

    #[id = 0x04]
    AcceptSudoMode(AcceptSudoMode),

    #[id = 0x05]
    DenySudoMode(DenySudoMode),

    #[id = 0x0A]
    AccessDenied(AccessDenied),

    #[id = 0x20]
    BlockData(BlockData),

    #[id = 0x21]
    AddNode(AddNode),

    #[id = 0x22]
    RemoveNode(RemoveNode),

    #[id = 0x27]
    Inventory(Inventory),

    #[id = 0x29]
    TimeOfDay(TimeOfDay),

    #[id = 0x2A]
    CsmRestrictionFlags(CsmRestrictionFlags),

    #[id = 0x2B]
    PlayerSpeed(PlayerSpeed),

    #[id = 0x2C]
    MediaPush(MediaPush),

    #[id = 0x2F]
    ChatMessage(ChatMessage),

    #[id = 0x31]
    ActiveObjectRemoveAdd(ActiveObjectRemoveAdd),

    #[id = 0x32]
    ActiveObjectMessages(ActiveObjectMessages),

    #[id = 0x33]
    Hp(Hp),

    #[id = 0x34]
    MovePlayer(MovePlayer),

    #[id = 0x35]
    AccessDeniedLegacy(AccessDeniedLegacy),

    #[id = 0x36]
    Fov(Fov),

    #[id = 0x37]
    DeathScreen(DeathScreen),

    #[id = 0x38]
    Media(Media),

    #[id = 0x3A]
    NodeDef(NodeDef),

    #[id = 0x3C]
    AnnounceMedia(AnnounceMedia),

    #[id = 0x3D]
    ItemDef(ItemDef),

    #[id = 0x3F]
    PlaySound(PlaySound),

    #[id = 0x40]
    StopSound(StopSound),

    #[id = 0x41]
    Privileges(Privileges),

    #[id = 0x42]
    InventoryFormSpec(InventoryFormSpec),

    #[id = 0x43]
    DetachedInventory(DetachedInventory),

    #[id = 0x44]
    ShowFormspec(ShowFormspec),

    #[id = 0x45]
    Movement(Movement),

    #[id = 0x46]
    SpawnParticle(SpawnParticle),

    #[id = 0x47]
    AddParticleSpawner(AddParticleSpawner),

    #[id = 0x49]
    HudAdd(HudAdd),

    #[id = 0x4A]
    HudRm(HudRm),

    #[id = 0x4B]
    HudChange(HudChange),

    #[id = 0x4E]
    Breath(Breath),

    #[id = 0x4F]
    SetSky(SetSky),

    #[id = 0x50]
    OverrideDayNightRatio(OverrideDayNightRatio),

    #[id = 0x51]
    LocalPlayerAnimations(LocalPlayerAnimations),

    #[id = 0x52]
    EyeOffset(EyeOffset),

    #[id = 0x53]
    DeleteParticleSpawner(DeleteParticleSpawner),

    #[id = 0x54]
    CloudParams(CloudParams),

    #[id = 0x55]
    FadeSound(FadeSound),

    #[id = 0x56]
    UpdatePlayerList(UpdatePlayerList),

    #[id = 0x57]
    ModChannelMsg(ModChannelMsg),

    #[id = 0x58]
    ModChannelSignal(ModChannelSignal),

    #[id = 0x59]
    NodeMetaChanged(NodeMetaChanged),

    #[id = 0x5A]
    SetSun(SetSun),

    #[id = 0x5B]
    SetMoon(SetMoon),

    #[id = 0x5C]
    SetStars(SetStars),

    #[id = 0x5D]
    MovePlayerRel(MovePlayerRel),

    #[id = 0x60]
    SrpBytesSB(SrpBytesSB),

    #[id = 0x61]
    FormspecPrepend(FormspecPrepend),

    #[id = 0x62]
    MinimapModes(MinimapModes),

    #[id = 0x63]
    SetLighting(SetLighting),
}

#[derive(Serialize, Debug)]
pub struct Hello {
    pub serialization_version: u8,
    pub compression_mode: u16,
    pub protocol_version: u16,
    pub auth_mechs: u32,
    pub legacy_name: String,
}

#[derive(Serialize, Debug)]
pub struct AuthAccept {}

#[derive(Serialize, Debug)]
pub struct AcceptSudoMode {}

#[derive(Serialize, Debug)]
pub struct DenySudoMode {}

#[derive(Serialize, Debug)]
pub struct AccessDenied {}

#[derive(Serialize, Debug)]
pub struct BlockData {}

#[derive(Serialize, Debug)]
pub struct AddNode {}

#[derive(Serialize, Debug)]
pub struct RemoveNode {}

#[derive(Serialize, Debug)]
pub struct Inventory {}

#[derive(Serialize, Debug)]
pub struct TimeOfDay {}

#[derive(Serialize, Debug)]
pub struct CsmRestrictionFlags {}

#[derive(Serialize, Debug)]
pub struct PlayerSpeed {}

#[derive(Serialize, Debug)]
pub struct MediaPush {}

#[derive(Serialize, Debug)]
pub struct ChatMessage {}

#[derive(Serialize, Debug)]
pub struct ActiveObjectRemoveAdd {}

#[derive(Serialize, Debug)]
pub struct ActiveObjectMessages {}

#[derive(Serialize, Debug)]
pub struct Hp {}

#[derive(Serialize, Debug)]
pub struct MovePlayer {}

#[derive(Serialize, Debug)]
pub struct AccessDeniedLegacy {}

#[derive(Serialize, Debug)]
pub struct Fov {}

#[derive(Serialize, Debug)]
pub struct DeathScreen {}

#[derive(Serialize, Debug)]
pub struct Media {}

#[derive(Serialize, Debug)]
pub struct NodeDef {}

#[derive(Serialize, Debug)]
pub struct AnnounceMedia {}

#[derive(Serialize, Debug)]
pub struct ItemDef {}

#[derive(Serialize, Debug)]
pub struct PlaySound {}

#[derive(Serialize, Debug)]
pub struct StopSound {}

#[derive(Serialize, Debug)]
pub struct Privileges {}

#[derive(Serialize, Debug)]
pub struct InventoryFormSpec {}

#[derive(Serialize, Debug)]
pub struct DetachedInventory {}

#[derive(Serialize, Debug)]
pub struct ShowFormspec {}

#[derive(Serialize, Debug)]
pub struct Movement {}

#[derive(Serialize, Debug)]
pub struct SpawnParticle {}

#[derive(Serialize, Debug)]
pub struct AddParticleSpawner {}

#[derive(Serialize, Debug)]
pub struct HudAdd {}

#[derive(Serialize, Debug)]
pub struct HudRm {}

#[derive(Serialize, Debug)]
pub struct HudChange {}

#[derive(Serialize, Debug)]
pub struct Breath {}

#[derive(Serialize, Debug)]
pub struct SetSky {}

#[derive(Serialize, Debug)]
pub struct OverrideDayNightRatio {}

#[derive(Serialize, Debug)]
pub struct LocalPlayerAnimations {}

#[derive(Serialize, Debug)]
pub struct EyeOffset {}

#[derive(Serialize, Debug)]
pub struct DeleteParticleSpawner {}

#[derive(Serialize, Debug)]
pub struct CloudParams {}

#[derive(Serialize, Debug)]
pub struct FadeSound {}

#[derive(Serialize, Debug)]
pub struct UpdatePlayerList {}

#[derive(Serialize, Debug)]
pub struct ModChannelMsg {}

#[derive(Serialize, Debug)]
pub struct ModChannelSignal {}

#[derive(Serialize, Debug)]
pub struct NodeMetaChanged {}

#[derive(Serialize, Debug)]
pub struct SetSun {}

#[derive(Serialize, Debug)]
pub struct SetMoon {}

#[derive(Serialize, Debug)]
pub struct SetStars {}

#[derive(Serialize, Debug)]
pub struct MovePlayerRel {}

#[derive(Serialize, Debug)]
pub struct SrpBytesSB {}

#[derive(Serialize, Debug)]
pub struct FormspecPrepend {}

#[derive(Serialize, Debug)]
pub struct MinimapModes {}

#[derive(Serialize, Debug)]
pub struct SetLighting {}
