use crate::{language::Dict, spell::SpellType};

/// 呪文を詠唱したときの動作を表します
/// 弾丸系魔法は Bullet にまとめられており、
/// そのほかの魔法も動作の種別によって分類されています
pub enum SpellCast {
    Bullet {
        slice: &'static str,

        collier_radius: f32,

        /// 魔法弾の速度
        /// pixels_per_meter が 100.0 に設定されているので、
        /// 200は1フレームに2ピクセル移動する速度です
        speed: f32,

        lifetime: u32,
        damage: i32,
        impulse: f32,

        scattering: f32,

        light_intensity: f32,
        light_radius: f32,
        light_color_hlsa: [f32; 4],
    },
    Heal,
    BulletSpeedUpDown {
        delta: f32,
    },
    MultipleCast {
        amount: u32,
    },
}

/// 呪文の基礎情報
pub struct SpellProps {
    pub name: Dict,
    pub description: Dict,
    pub mana_drain: i32,
    pub cast_delay: u32,
    pub icon: &'static str,
    pub cast: SpellCast,
}

const MAGIC_BOLT: SpellProps = SpellProps {
    name: Dict {
        ja: "マジックボルト",
        en: "Magic Bolt",
    },
    description: Dict {
        ja: "魔力の塊を発射する、最も基本的な攻撃魔法です。",
        en: "A basic attack spell that fires a bolt of magic.",
    },
    mana_drain: 50,
    cast_delay: 10,
    icon: "bullet_magic_bolt",
    cast: SpellCast::Bullet {
        slice: "bullet_magic_bolt",
        collier_radius: 5.0,
        speed: 100.0,
        lifetime: 240,
        damage: 8,
        impulse: 20000.0,
        scattering: 0.4,
        light_intensity: 1.0,
        light_radius: 50.0,
        light_color_hlsa: [245.0, 1.0, 0.6, 1.0],
    },
};

const PURPLE_BOLT: SpellProps = SpellProps {
    name: Dict {
        ja: "悪意の視線",
        en: "Evil Eye",
    },
    description: Dict {
        ja:
            "邪悪な魔力を帯びた視線です。浴びせられると少し嫌な気持ちになります。",
        en: "Fires a slow-moving purple energy bolt. It is weak but consumes little mana.",
    },
    mana_drain: 10,
    cast_delay: 120,
    icon: "bullet_purple",
    cast: SpellCast::Bullet {
        slice: "bullet_purple",
        collier_radius: 5.0,
        speed: 50.0,
        lifetime: 500,
        damage: 3,
        impulse: 0.0,
        scattering: 0.6,
        light_intensity: 0.0,
        light_radius: 0.0,
        light_color_hlsa: [0.0, 0.0, 0.0, 1.0],
    },
};

const SLIME_CHARGE: SpellProps = SpellProps {
    name: Dict {
        ja: "スライムの塊",
        en: "Slime Limp",
    },
    description: Dict { 
        ja: "ぷにぷにとした塊で殴りつけます。痛くはありませんが、相手を大きく吹き飛ばします。",
        en: "Slap with a soft and squishy lump. It doesn't hurt much, but it blows the opponent away."
    },
    mana_drain: 200,
    cast_delay: 30,
    icon: "bullet_slime_charge",
    cast: SpellCast::Bullet {
        slice: "bullet_slime_charge",
        collier_radius: 5.0,
        speed: 2.0,
        lifetime: 5,
        damage: 1,
        impulse: 40000.0,
        scattering: 0.0,
        light_intensity: 0.0,
        light_radius: 0.0,
        light_color_hlsa: [0.0, 0.0, 0.0, 1.0],
    },
};

const HEAL: SpellProps = SpellProps {
    name: Dict {
        ja: "回復",
        en: "Heal",
    },
    description: Dict { ja: "自分自身の体力を少しだけ回復します。", 
    en: "Heals a small amount of your own health." },
    mana_drain: 20,
    cast_delay: 120,
    icon: "spell_heal",
    cast: SpellCast::Heal,
};

const BULLET_SPEED_UP: SpellProps = SpellProps {
    name: Dict {
        ja: "加速",
        en: "Speed Up",
    },
    description: Dict { ja: "次に発射する魔法の弾速を50%上昇させます。",
    en: "Increases the speed of the next magic bullet by 50%." },
    mana_drain: 20,
    cast_delay: 0,
    icon: "bullet_speed_up",
    cast: SpellCast::BulletSpeedUpDown { delta: 0.5 },
};

const BULLET_SPEED_DOWN: SpellProps = SpellProps {
    name: Dict {
        ja: "減速",
        en: "Speed Down",
    },
    description: Dict { ja: "次に発射する魔法の弾速を50%低下させます。",
    en: "Reduces the speed of the next magic bullet by 50%." },
    mana_drain: 20,
    cast_delay: 0,
    icon: "bullet_speed_down",
    cast: SpellCast::BulletSpeedUpDown { delta: -0.5 },
};

const DUAL_CAST: SpellProps = SpellProps {
    name: Dict {
        ja: "二重呪文",
        en: "Dual Cast",
    },
    description: Dict { ja: "ふたつの呪文を同時に詠唱します。", 
    en: "Casts two spells at the same time." },
    mana_drain: 20,
    cast_delay: 0,
    icon: "spell_dual_cast",
    cast: SpellCast::MultipleCast { amount: 2 },
};

const TRIPLE_CAST: SpellProps = SpellProps {
    name: Dict {
        ja: "三重呪文",
        en: "Triple Cast",
    },
    description: Dict { ja: "みっつの呪文を同時に詠唱します。", 
    en: "Casts three spells at the same time." },
    mana_drain: 20,
    cast_delay: 0,
    icon: "spell_triple_cast",
    cast: SpellCast::MultipleCast { amount: 3 },
};

pub fn spell_to_props(spell: SpellType) -> SpellProps {
    match spell {
        SpellType::MagicBolt => MAGIC_BOLT,
        SpellType::PurpleBolt => PURPLE_BOLT,
        SpellType::SlimeCharge => SLIME_CHARGE,
        SpellType::Heal => HEAL,
        SpellType::BulletSpeedUp => BULLET_SPEED_UP,
        SpellType::BulletSpeedDoown => BULLET_SPEED_DOWN,
        SpellType::DualCast => DUAL_CAST,
        SpellType::TripleCast => TRIPLE_CAST,
    }
}

pub fn get_spell_appendix(cast: SpellCast) -> String {
    match cast {
        SpellCast::Bullet {
            slice: _,
            collier_radius,
            speed,
            lifetime,
            damage,
            impulse,
            scattering,
            light_intensity: _,
            light_radius: _,
            light_color_hlsa: _,
        } => {
            format!(
                "ダメージ:{}  ノックバック:{}\n射出速度:{}  持続時間:{}\n拡散:{}  大きさ:{}",
                damage,
                impulse * 0.001,
                speed,
                lifetime,
                scattering,
                collier_radius,
            )
        }
        SpellCast::Heal => {
            format!("回復:{}", 10)
        }
        SpellCast::BulletSpeedUpDown { delta: _ } => format!(""),
        SpellCast::MultipleCast { amount: _ } => format!(""),
    }
}
