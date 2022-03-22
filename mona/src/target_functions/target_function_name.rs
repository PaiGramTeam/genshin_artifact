use num_derive::FromPrimitive;
use serde::{Serialize, Deserialize};
use mona_derive::{TargetFunctionData, EnumLen};
use strum_macros::Display;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[derive(TargetFunctionData, EnumLen, FromPrimitive, Display)]
pub enum TargetFunctionName {
    MaxATK,
    MaxDEF,
    MaxHP,
    MaxEM,
    PyroDamage,
    CryoDamage,
    HydroDamage,
    ElectroDamage,
    AnemoDamage,
    GeoDamage,
    PhysicalDamage,
    MaxVaporize,
    MaxMelt,

    AlbedoDefault,
    AloyDefault,
    AmberDefault,
    AratakiIttoDefault,
    BarbaraDefault,
    BeidouDefault,
    BennettDamage,
    BennettDefault,
    ChongyunDefault,
    DilucDefault,
    DionaDefault,
    EulaDefault,
    FischlDefault,
    GanyuDefault,
    GorouDefault,
    HuTaoDefault,
    JeanDefault,
    KaedeharaKazuhaDamage,
    KaedeharaKazuhaDefault,
    KaeyaDefault,
    KamisatoAyakaDefault,
    KamisatoAyakaDps,
    KeqingDefault,
    KleeDefault,
    KujouSaraDamage,
    KujouSaraDefault,
    LisaDefault,
    MonaDefault,
    NingguangDefault,
    NoelleDefault,
    QiqiDefault,
    RaidenShogunDefault,
    RazorDefault,
    RosariaDefault,
    SangonomiyaKokomiDefault,
    SayuDefault,
    ShenheDefault,
    SucroseDefault,
    TartagliaDefault,
    ThomaDefault,
    VentiDefault,
    XianglingDefault,
    XiaoDefault,
    XingqiuDefault,
    XinyanDefault,
    YaeMikoDefault,
    YanfeiDefault,
    YoimiyaDefault,
    YunjinDefault,
    ZhongliDefault,
}
