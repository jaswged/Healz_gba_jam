use agb::include_wav;
use agb::fixnum::num;
use agb::rng;
use agb::sound::mixer::{ChannelId, Mixer, SoundChannel};

// Music
static BATTLE_A: &[u8] = include_wav!("music/battleA.wav");
static BATTLE_B: &[u8] = include_wav!("music/battleB.wav");
static VICTORY: &[u8] = include_wav!("music/victory.wav");
static GAME_OVER: &[u8] = include_wav!("music/game_over.wav");


// region Sound Effects
// static CREATURE_SMALL: &[u8] = include_wav!("sfx/creature_small.wav");
// static CREATURES: &[u8] = include_wav!("sfx/creatures.wav");
static FIRE_HIT_01: &[u8] = include_wav!("sfx/Fire_Hit_01.wav");
static FIRE_HIT_02: &[u8] = include_wav!("sfx/Fire_Hit_02.wav");
static DEATH_1: &[u8] = include_wav!("sfx/Human_Die01.wav");
static DEATH_2: &[u8] = include_wav!("sfx/Human_Die02.wav");
static MAGIC_HIT: &[u8] = include_wav!("sfx/Magic_Hit.wav");
// static MAGIC_HIT_MULTIPLE: &[u8] = include_wav!("sfx/Magic_Hit_Multiple.wav");
static MENU_CLICK: &[u8] = include_wav!("sfx/Menu_Click.wav");
static SFX_DAMAGE_HIT3: &[u8] = include_wav!("sfx/sfx_damage_hit3.wav");
static SFX_MENU_MOVE4: &[u8] = include_wav!("sfx/sfx_menu_move4.wav");
// static SFX_SOUNDS_ERROR1: &[u8] = include_wav!("sfx/sfx_sounds_error1.wav");
// static SFX_SOUNDS_ERROR2: &[u8] = include_wav!("sfx/sfx_sounds_error2.wav");
static SFX_SOUNDS_PAUSE1_IN: &[u8] = include_wav!("sfx/sfx_sounds_pause1_in.wav");
static SFX_SOUNDS_PAUSE1_OUT: &[u8] = include_wav!("sfx/sfx_sounds_pause1_out.wav");
static OOM: &[u8] = include_wav!("sfx/oom.wav");
static SWORD_ATK: &[u8] = include_wav!("sfx/sword_atk.wav");
static SWORD_HIT_1: &[u8] = include_wav!("sfx/SWORD_HIT_1.wav");
static SWORD_HIT_2: &[u8] = include_wav!("sfx/sword_hit_2.wav");
// Endregion

pub struct Sfx<'a> {
    bgm: ChannelId,
    mixer: &'a mut Mixer<'a>,
}

impl<'a> Sfx<'a> {
    pub fn new(mixer: &'a mut Mixer<'a>) -> Self {
        let mut title_music = SoundChannel::new_high_priority(BATTLE_B);
        title_music.should_loop();
        let title_channel = mixer.play_sound(title_music).unwrap();

        Self {
            mixer,
            bgm: title_channel,
        }
    }

    pub fn title_screen(&mut self) {
        self.mixer.channel(&self.bgm).unwrap().stop();

        let mut title_music = SoundChannel::new_high_priority(BATTLE_B);
        title_music.should_loop();
        self.bgm = self.mixer.play_sound(title_music).unwrap();
    }

    pub fn frame(&mut self) {
        self.mixer.frame();
    }

    pub fn stop_music(&mut self) {
        let channel = self.mixer.channel(&self.bgm).unwrap();
        channel.stop();
    }

    pub fn boss(&mut self) {
        let channel = self.mixer.channel(&self.bgm).unwrap();
        channel.stop();

        let mut channel = SoundChannel::new_high_priority(BATTLE_A);
        channel.stereo().should_loop();
        self.bgm = self.mixer.play_sound(channel).unwrap();
    }

    pub fn game_over(&mut self) {
        let channel = self.mixer.channel(&self.bgm).unwrap();
        channel.stop();

        let mut channel = SoundChannel::new_high_priority(GAME_OVER);
        channel.stereo().should_loop();
        self.bgm = self.mixer.play_sound(channel).unwrap();
    }

    pub fn victory(&mut self) {
        let channel = self.mixer.channel(&self.bgm).unwrap();
        channel.stop();

        let mut channel = SoundChannel::new_high_priority(VICTORY);
        channel.stereo().should_loop();
        self.bgm = self.mixer.play_sound(channel).unwrap();
    }

    // Sound Effect Functions
    pub fn player_died(&mut self) {
        let is_first = rng::gen() as usize % 2;
        if is_first == 0 {
            self.mixer.play_sound(SoundChannel::new(DEATH_1));
        } else {
            self.mixer.play_sound(SoundChannel::new(DEATH_2));
        }
    }

    pub fn player_heal(&mut self) {
        self.mixer.play_sound(SoundChannel::new(MAGIC_HIT));
    }

    pub fn player_oom(&mut self) {
        self.mixer.play_sound(SoundChannel::new(OOM));
    }

    pub fn text_speed(&mut self) {
        self.mixer.play_sound(SoundChannel::new(MENU_CLICK));
    }

    pub fn hot_ready(&mut self) {
        self.mixer.play_sound(SoundChannel::new(SFX_MENU_MOVE4));
    }

    pub fn cauterize_ready(&mut self) {
        self.mixer.play_sound(SoundChannel::new(SFX_MENU_MOVE4));
    }

    pub fn fire_hit(&mut self) {
        // alternate fire 1 & 2 sounds
        let is_first = rng::gen() as usize % 2;
        if is_first == 0 {
            self.mixer.play_sound(SoundChannel::new(FIRE_HIT_01));
        } else {
            self.mixer.play_sound(SoundChannel::new(FIRE_HIT_02));
        }
    }

    pub fn sword_sound(&mut self) {
        let is_first = rng::gen() as usize % 5;
        if  is_first == 0 || is_first == 5 {
            self.mixer.play_sound(SoundChannel::new(SWORD_ATK));
        } else if is_first == 1 {
            self.mixer.play_sound(SoundChannel::new(SWORD_HIT_1));
        } else if is_first == 2 {
            self.mixer.play_sound(SoundChannel::new(SFX_DAMAGE_HIT3));
        }else {
            self.mixer.play_sound(SoundChannel::new(SWORD_HIT_2));
        }
    }

    pub fn pause(&mut self) {
        self.mixer.play_sound(SoundChannel::new(SFX_SOUNDS_PAUSE1_IN));
    }
    pub fn unpause(&mut self) {
        self.mixer.play_sound(SoundChannel::new(SFX_SOUNDS_PAUSE1_OUT));
    }
}