use agb::include_wav;
use agb::fixnum::num;
use agb::rng;
use agb::sound::mixer::{ChannelId, Mixer, SoundChannel};


static BATTLE_A: &[u8] = include_wav!("music/battle.wav");
static BOSS_BATTLE_1: &[u8] = include_wav!("music/boss_battle_1.wav");
static FANTASY_2: &[u8] = include_wav!("music/fantasy_2.wav");
static EPIC: &[u8] = include_wav!("music/epic.wav");
// static BOSS_BATTLE_2: &[u8] = include_wav!("music/boss_battle_2.wav");
// static BOSS_BATTLE_3: &[u8] = include_wav!("music/boss_battle_3.wav"); // TItle
// static BOSS_BATTLE_4: &[u8] = include_wav!("music/boss_battle_4.wav");
// static FANTASY_1: &[u8] = include_wav!("music/fantasy_1.wav");

static DEATH_1: &[u8] = include_wav!("sfx/death_1.wav");
static SWORD_1: &[u8] = include_wav!("sfx/sword_hit_1.wav");
static OOM: &[u8] = include_wav!("sfx/oom.wav");
static READY: &[u8] = include_wav!("sfx/ready.wav");
static FIRE_HIT: &[u8] = include_wav!("sfx/Fire_Hit_01.wav");
static TEXT: &[u8] = include_wav!("sfx/text_fast.wav");
static GAME_OVER: &[u8] = include_wav!("sfx/game_over.wav");
static BOOK: &[u8] = include_wav!("sfx/book.wav");


pub struct Sfx<'a> {
    bgm: ChannelId,
    mixer: &'a mut Mixer<'a>,
}

impl<'a> Sfx<'a> {
    pub fn new(mixer: &'a mut Mixer<'a>) -> Self {
        let mut title_music = SoundChannel::new_high_priority(FANTASY_2);
        title_music.should_loop();
        let title_channel = mixer.play_sound(title_music).unwrap();

        Self {
            mixer,
            bgm: title_channel,
        }
    }

    pub fn title_screen(&mut self) {
        self.mixer.channel(&self.bgm).unwrap().stop();

        let mut title_music = SoundChannel::new_high_priority(FANTASY_2);
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

        let mut channel = SoundChannel::new(BATTLE_A); //_high_priority
        channel.stereo().should_loop();
        self.bgm = self.mixer.play_sound(channel).unwrap();
    }

    pub fn game_over(&mut self) {
        self.mixer.play_sound(SoundChannel::new(GAME_OVER));
    }

    pub fn sword(&mut self) {
        self.mixer.play_sound(SoundChannel::new(SWORD_1));
    }

    pub fn player_died(&mut self) {
        self.mixer.play_sound(SoundChannel::new(DEATH_1));
    }

    pub fn player_heal(&mut self) {
        self.mixer.play_sound(SoundChannel::new(SWORD_1));
    }

    pub fn player_oom(&mut self) {
        self.mixer.play_sound(SoundChannel::new(OOM));
    }

    pub fn text_speed(&mut self) {
        self.mixer.play_sound(SoundChannel::new(TEXT));
    }

    pub fn hot_ready(&mut self) {
        self.mixer.play_sound(SoundChannel::new(READY));
    }

    pub fn fire_hit(&mut self) {
        self.mixer.play_sound(SoundChannel::new(FIRE_HIT));
    }

    pub fn book(&mut self) {
        self.mixer.play_sound(SoundChannel::new(BOOK));
    }
}