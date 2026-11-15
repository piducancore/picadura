use truce::prelude::*;

#[derive(ParamEnum)]
pub enum Scale {
    Minor,
    Dorian,
    Phrygian,
    Major,
    Pentatonic,
    Chromatic,
}

#[derive(Params)]
pub struct PicaduraParams {
    #[param(name = "Run", default = true)]
    pub run: BoolParam,
    #[param(name = "Root", range = "discrete(24, 72)", default = 36)]
    pub root: IntParam,
    #[param(name = "Scale")]
    pub scale: EnumParam<Scale>,
    #[param(name = "Steps", range = "discrete(1, 64)", default = 16)]
    pub length: IntParam,
    #[param(name = "Density", range = "linear(0, 100)", default = 70)]
    pub density: FloatParam,
    #[param(name = "Accent", range = "linear(0, 100)", default = 30)]
    pub accent: FloatParam,
    #[param(name = "Gate", range = "linear(5, 95)", default = 55)]
    pub gate: FloatParam,
    #[param(name = "Slide", range = "linear(0, 100)", default = 20)]
    pub slide: FloatParam,
    #[param(name = "Swing", range = "linear(0, 60)", default = 0)]
    pub swing: FloatParam,
    #[param(name = "Octaves", range = "discrete(1, 3)", default = 2)]
    pub octaves: IntParam,
    #[param(name = "Notes Seed", range = "discrete(0, 65535)", default = 1)]
    pub notes_seed: IntParam,
    #[param(name = "Rhythm Seed", range = "discrete(0, 65535)", default = 7)]
    pub rhythm_seed: IntParam,
    #[param(name = "Expression Seed", range = "discrete(0, 65535)", default = 11)]
    pub expression_seed: IntParam,
    #[param(name = "Step 01 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_0: IntParam,
    #[param(name = "Step 02 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_1: IntParam,
    #[param(name = "Step 03 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_2: IntParam,
    #[param(name = "Step 04 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_3: IntParam,
    #[param(name = "Step 05 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_4: IntParam,
    #[param(name = "Step 06 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_5: IntParam,
    #[param(name = "Step 07 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_6: IntParam,
    #[param(name = "Step 08 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_7: IntParam,
    #[param(name = "Step 09 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_8: IntParam,
    #[param(name = "Step 10 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_9: IntParam,
    #[param(name = "Step 11 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_10: IntParam,
    #[param(name = "Step 12 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_11: IntParam,
    #[param(name = "Step 13 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_12: IntParam,
    #[param(name = "Step 14 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_13: IntParam,
    #[param(name = "Step 15 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_14: IntParam,
    #[param(name = "Step 16 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_15: IntParam,
    #[param(name = "Step 17 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_16: IntParam,
    #[param(name = "Step 18 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_17: IntParam,
    #[param(name = "Step 19 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_18: IntParam,
    #[param(name = "Step 20 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_19: IntParam,
    #[param(name = "Step 21 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_20: IntParam,
    #[param(name = "Step 22 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_21: IntParam,
    #[param(name = "Step 23 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_22: IntParam,
    #[param(name = "Step 24 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_23: IntParam,
    #[param(name = "Step 25 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_24: IntParam,
    #[param(name = "Step 26 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_25: IntParam,
    #[param(name = "Step 27 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_26: IntParam,
    #[param(name = "Step 28 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_27: IntParam,
    #[param(name = "Step 29 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_28: IntParam,
    #[param(name = "Step 30 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_29: IntParam,
    #[param(name = "Step 31 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_30: IntParam,
    #[param(name = "Step 32 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_31: IntParam,
    #[param(name = "Step 33 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_32: IntParam,
    #[param(name = "Step 34 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_33: IntParam,
    #[param(name = "Step 35 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_34: IntParam,
    #[param(name = "Step 36 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_35: IntParam,
    #[param(name = "Step 37 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_36: IntParam,
    #[param(name = "Step 38 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_37: IntParam,
    #[param(name = "Step 39 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_38: IntParam,
    #[param(name = "Step 40 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_39: IntParam,
    #[param(name = "Step 41 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_40: IntParam,
    #[param(name = "Step 42 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_41: IntParam,
    #[param(name = "Step 43 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_42: IntParam,
    #[param(name = "Step 44 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_43: IntParam,
    #[param(name = "Step 45 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_44: IntParam,
    #[param(name = "Step 46 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_45: IntParam,
    #[param(name = "Step 47 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_46: IntParam,
    #[param(name = "Step 48 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_47: IntParam,
    #[param(name = "Step 49 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_48: IntParam,
    #[param(name = "Step 50 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_49: IntParam,
    #[param(name = "Step 51 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_50: IntParam,
    #[param(name = "Step 52 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_51: IntParam,
    #[param(name = "Step 53 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_52: IntParam,
    #[param(name = "Step 54 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_53: IntParam,
    #[param(name = "Step 55 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_54: IntParam,
    #[param(name = "Step 56 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_55: IntParam,
    #[param(name = "Step 57 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_56: IntParam,
    #[param(name = "Step 58 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_57: IntParam,
    #[param(name = "Step 59 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_58: IntParam,
    #[param(name = "Step 60 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_59: IntParam,
    #[param(name = "Step 61 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_60: IntParam,
    #[param(name = "Step 62 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_61: IntParam,
    #[param(name = "Step 63 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_62: IntParam,
    #[param(name = "Step 64 Note", range = "discrete(-2, 127)", default = -2)]
    pub step_63: IntParam,
    #[param(name = "Step 01 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_0: FloatParam,
    #[param(name = "Step 02 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_1: FloatParam,
    #[param(name = "Step 03 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_2: FloatParam,
    #[param(name = "Step 04 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_3: FloatParam,
    #[param(name = "Step 05 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_4: FloatParam,
    #[param(name = "Step 06 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_5: FloatParam,
    #[param(name = "Step 07 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_6: FloatParam,
    #[param(name = "Step 08 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_7: FloatParam,
    #[param(name = "Step 09 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_8: FloatParam,
    #[param(name = "Step 10 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_9: FloatParam,
    #[param(name = "Step 11 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_10: FloatParam,
    #[param(name = "Step 12 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_11: FloatParam,
    #[param(name = "Step 13 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_12: FloatParam,
    #[param(name = "Step 14 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_13: FloatParam,
    #[param(name = "Step 15 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_14: FloatParam,
    #[param(name = "Step 16 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_15: FloatParam,
    #[param(name = "Step 17 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_16: FloatParam,
    #[param(name = "Step 18 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_17: FloatParam,
    #[param(name = "Step 19 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_18: FloatParam,
    #[param(name = "Step 20 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_19: FloatParam,
    #[param(name = "Step 21 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_20: FloatParam,
    #[param(name = "Step 22 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_21: FloatParam,
    #[param(name = "Step 23 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_22: FloatParam,
    #[param(name = "Step 24 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_23: FloatParam,
    #[param(name = "Step 25 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_24: FloatParam,
    #[param(name = "Step 26 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_25: FloatParam,
    #[param(name = "Step 27 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_26: FloatParam,
    #[param(name = "Step 28 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_27: FloatParam,
    #[param(name = "Step 29 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_28: FloatParam,
    #[param(name = "Step 30 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_29: FloatParam,
    #[param(name = "Step 31 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_30: FloatParam,
    #[param(name = "Step 32 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_31: FloatParam,
    #[param(name = "Step 33 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_32: FloatParam,
    #[param(name = "Step 34 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_33: FloatParam,
    #[param(name = "Step 35 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_34: FloatParam,
    #[param(name = "Step 36 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_35: FloatParam,
    #[param(name = "Step 37 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_36: FloatParam,
    #[param(name = "Step 38 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_37: FloatParam,
    #[param(name = "Step 39 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_38: FloatParam,
    #[param(name = "Step 40 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_39: FloatParam,
    #[param(name = "Step 41 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_40: FloatParam,
    #[param(name = "Step 42 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_41: FloatParam,
    #[param(name = "Step 43 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_42: FloatParam,
    #[param(name = "Step 44 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_43: FloatParam,
    #[param(name = "Step 45 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_44: FloatParam,
    #[param(name = "Step 46 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_45: FloatParam,
    #[param(name = "Step 47 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_46: FloatParam,
    #[param(name = "Step 48 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_47: FloatParam,
    #[param(name = "Step 49 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_48: FloatParam,
    #[param(name = "Step 50 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_49: FloatParam,
    #[param(name = "Step 51 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_50: FloatParam,
    #[param(name = "Step 52 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_51: FloatParam,
    #[param(name = "Step 53 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_52: FloatParam,
    #[param(name = "Step 54 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_53: FloatParam,
    #[param(name = "Step 55 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_54: FloatParam,
    #[param(name = "Step 56 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_55: FloatParam,
    #[param(name = "Step 57 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_56: FloatParam,
    #[param(name = "Step 58 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_57: FloatParam,
    #[param(name = "Step 59 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_58: FloatParam,
    #[param(name = "Step 60 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_59: FloatParam,
    #[param(name = "Step 61 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_60: FloatParam,
    #[param(name = "Step 62 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_61: FloatParam,
    #[param(name = "Step 63 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_62: FloatParam,
    #[param(name = "Step 64 Timing", range = "linear(-25, 25)", default = 0)]
    pub timing_63: FloatParam,
}
impl PicaduraParams {
    pub fn overrides(&self) -> [i32; 64] {
        [
            self.step_0.value_i32(),
            self.step_1.value_i32(),
            self.step_2.value_i32(),
            self.step_3.value_i32(),
            self.step_4.value_i32(),
            self.step_5.value_i32(),
            self.step_6.value_i32(),
            self.step_7.value_i32(),
            self.step_8.value_i32(),
            self.step_9.value_i32(),
            self.step_10.value_i32(),
            self.step_11.value_i32(),
            self.step_12.value_i32(),
            self.step_13.value_i32(),
            self.step_14.value_i32(),
            self.step_15.value_i32(),
            self.step_16.value_i32(),
            self.step_17.value_i32(),
            self.step_18.value_i32(),
            self.step_19.value_i32(),
            self.step_20.value_i32(),
            self.step_21.value_i32(),
            self.step_22.value_i32(),
            self.step_23.value_i32(),
            self.step_24.value_i32(),
            self.step_25.value_i32(),
            self.step_26.value_i32(),
            self.step_27.value_i32(),
            self.step_28.value_i32(),
            self.step_29.value_i32(),
            self.step_30.value_i32(),
            self.step_31.value_i32(),
            self.step_32.value_i32(),
            self.step_33.value_i32(),
            self.step_34.value_i32(),
            self.step_35.value_i32(),
            self.step_36.value_i32(),
            self.step_37.value_i32(),
            self.step_38.value_i32(),
            self.step_39.value_i32(),
            self.step_40.value_i32(),
            self.step_41.value_i32(),
            self.step_42.value_i32(),
            self.step_43.value_i32(),
            self.step_44.value_i32(),
            self.step_45.value_i32(),
            self.step_46.value_i32(),
            self.step_47.value_i32(),
            self.step_48.value_i32(),
            self.step_49.value_i32(),
            self.step_50.value_i32(),
            self.step_51.value_i32(),
            self.step_52.value_i32(),
            self.step_53.value_i32(),
            self.step_54.value_i32(),
            self.step_55.value_i32(),
            self.step_56.value_i32(),
            self.step_57.value_i32(),
            self.step_58.value_i32(),
            self.step_59.value_i32(),
            self.step_60.value_i32(),
            self.step_61.value_i32(),
            self.step_62.value_i32(),
            self.step_63.value_i32(),
        ]
    }
    pub fn timings(&self) -> [f64; 64] {
        [
            f64::from(self.timing_0.value()) / 100.0,
            f64::from(self.timing_1.value()) / 100.0,
            f64::from(self.timing_2.value()) / 100.0,
            f64::from(self.timing_3.value()) / 100.0,
            f64::from(self.timing_4.value()) / 100.0,
            f64::from(self.timing_5.value()) / 100.0,
            f64::from(self.timing_6.value()) / 100.0,
            f64::from(self.timing_7.value()) / 100.0,
            f64::from(self.timing_8.value()) / 100.0,
            f64::from(self.timing_9.value()) / 100.0,
            f64::from(self.timing_10.value()) / 100.0,
            f64::from(self.timing_11.value()) / 100.0,
            f64::from(self.timing_12.value()) / 100.0,
            f64::from(self.timing_13.value()) / 100.0,
            f64::from(self.timing_14.value()) / 100.0,
            f64::from(self.timing_15.value()) / 100.0,
            f64::from(self.timing_16.value()) / 100.0,
            f64::from(self.timing_17.value()) / 100.0,
            f64::from(self.timing_18.value()) / 100.0,
            f64::from(self.timing_19.value()) / 100.0,
            f64::from(self.timing_20.value()) / 100.0,
            f64::from(self.timing_21.value()) / 100.0,
            f64::from(self.timing_22.value()) / 100.0,
            f64::from(self.timing_23.value()) / 100.0,
            f64::from(self.timing_24.value()) / 100.0,
            f64::from(self.timing_25.value()) / 100.0,
            f64::from(self.timing_26.value()) / 100.0,
            f64::from(self.timing_27.value()) / 100.0,
            f64::from(self.timing_28.value()) / 100.0,
            f64::from(self.timing_29.value()) / 100.0,
            f64::from(self.timing_30.value()) / 100.0,
            f64::from(self.timing_31.value()) / 100.0,
            f64::from(self.timing_32.value()) / 100.0,
            f64::from(self.timing_33.value()) / 100.0,
            f64::from(self.timing_34.value()) / 100.0,
            f64::from(self.timing_35.value()) / 100.0,
            f64::from(self.timing_36.value()) / 100.0,
            f64::from(self.timing_37.value()) / 100.0,
            f64::from(self.timing_38.value()) / 100.0,
            f64::from(self.timing_39.value()) / 100.0,
            f64::from(self.timing_40.value()) / 100.0,
            f64::from(self.timing_41.value()) / 100.0,
            f64::from(self.timing_42.value()) / 100.0,
            f64::from(self.timing_43.value()) / 100.0,
            f64::from(self.timing_44.value()) / 100.0,
            f64::from(self.timing_45.value()) / 100.0,
            f64::from(self.timing_46.value()) / 100.0,
            f64::from(self.timing_47.value()) / 100.0,
            f64::from(self.timing_48.value()) / 100.0,
            f64::from(self.timing_49.value()) / 100.0,
            f64::from(self.timing_50.value()) / 100.0,
            f64::from(self.timing_51.value()) / 100.0,
            f64::from(self.timing_52.value()) / 100.0,
            f64::from(self.timing_53.value()) / 100.0,
            f64::from(self.timing_54.value()) / 100.0,
            f64::from(self.timing_55.value()) / 100.0,
            f64::from(self.timing_56.value()) / 100.0,
            f64::from(self.timing_57.value()) / 100.0,
            f64::from(self.timing_58.value()) / 100.0,
            f64::from(self.timing_59.value()) / 100.0,
            f64::from(self.timing_60.value()) / 100.0,
            f64::from(self.timing_61.value()) / 100.0,
            f64::from(self.timing_62.value()) / 100.0,
            f64::from(self.timing_63.value()) / 100.0,
        ]
    }
}
pub const NOTE_IDS: [PicaduraParamsParamId; 64] = [
    PicaduraParamsParamId::Step0,
    PicaduraParamsParamId::Step1,
    PicaduraParamsParamId::Step2,
    PicaduraParamsParamId::Step3,
    PicaduraParamsParamId::Step4,
    PicaduraParamsParamId::Step5,
    PicaduraParamsParamId::Step6,
    PicaduraParamsParamId::Step7,
    PicaduraParamsParamId::Step8,
    PicaduraParamsParamId::Step9,
    PicaduraParamsParamId::Step10,
    PicaduraParamsParamId::Step11,
    PicaduraParamsParamId::Step12,
    PicaduraParamsParamId::Step13,
    PicaduraParamsParamId::Step14,
    PicaduraParamsParamId::Step15,
    PicaduraParamsParamId::Step16,
    PicaduraParamsParamId::Step17,
    PicaduraParamsParamId::Step18,
    PicaduraParamsParamId::Step19,
    PicaduraParamsParamId::Step20,
    PicaduraParamsParamId::Step21,
    PicaduraParamsParamId::Step22,
    PicaduraParamsParamId::Step23,
    PicaduraParamsParamId::Step24,
    PicaduraParamsParamId::Step25,
    PicaduraParamsParamId::Step26,
    PicaduraParamsParamId::Step27,
    PicaduraParamsParamId::Step28,
    PicaduraParamsParamId::Step29,
    PicaduraParamsParamId::Step30,
    PicaduraParamsParamId::Step31,
    PicaduraParamsParamId::Step32,
    PicaduraParamsParamId::Step33,
    PicaduraParamsParamId::Step34,
    PicaduraParamsParamId::Step35,
    PicaduraParamsParamId::Step36,
    PicaduraParamsParamId::Step37,
    PicaduraParamsParamId::Step38,
    PicaduraParamsParamId::Step39,
    PicaduraParamsParamId::Step40,
    PicaduraParamsParamId::Step41,
    PicaduraParamsParamId::Step42,
    PicaduraParamsParamId::Step43,
    PicaduraParamsParamId::Step44,
    PicaduraParamsParamId::Step45,
    PicaduraParamsParamId::Step46,
    PicaduraParamsParamId::Step47,
    PicaduraParamsParamId::Step48,
    PicaduraParamsParamId::Step49,
    PicaduraParamsParamId::Step50,
    PicaduraParamsParamId::Step51,
    PicaduraParamsParamId::Step52,
    PicaduraParamsParamId::Step53,
    PicaduraParamsParamId::Step54,
    PicaduraParamsParamId::Step55,
    PicaduraParamsParamId::Step56,
    PicaduraParamsParamId::Step57,
    PicaduraParamsParamId::Step58,
    PicaduraParamsParamId::Step59,
    PicaduraParamsParamId::Step60,
    PicaduraParamsParamId::Step61,
    PicaduraParamsParamId::Step62,
    PicaduraParamsParamId::Step63,
];
pub const TIMING_IDS: [PicaduraParamsParamId; 64] = [
    PicaduraParamsParamId::Timing0,
    PicaduraParamsParamId::Timing1,
    PicaduraParamsParamId::Timing2,
    PicaduraParamsParamId::Timing3,
    PicaduraParamsParamId::Timing4,
    PicaduraParamsParamId::Timing5,
    PicaduraParamsParamId::Timing6,
    PicaduraParamsParamId::Timing7,
    PicaduraParamsParamId::Timing8,
    PicaduraParamsParamId::Timing9,
    PicaduraParamsParamId::Timing10,
    PicaduraParamsParamId::Timing11,
    PicaduraParamsParamId::Timing12,
    PicaduraParamsParamId::Timing13,
    PicaduraParamsParamId::Timing14,
    PicaduraParamsParamId::Timing15,
    PicaduraParamsParamId::Timing16,
    PicaduraParamsParamId::Timing17,
    PicaduraParamsParamId::Timing18,
    PicaduraParamsParamId::Timing19,
    PicaduraParamsParamId::Timing20,
    PicaduraParamsParamId::Timing21,
    PicaduraParamsParamId::Timing22,
    PicaduraParamsParamId::Timing23,
    PicaduraParamsParamId::Timing24,
    PicaduraParamsParamId::Timing25,
    PicaduraParamsParamId::Timing26,
    PicaduraParamsParamId::Timing27,
    PicaduraParamsParamId::Timing28,
    PicaduraParamsParamId::Timing29,
    PicaduraParamsParamId::Timing30,
    PicaduraParamsParamId::Timing31,
    PicaduraParamsParamId::Timing32,
    PicaduraParamsParamId::Timing33,
    PicaduraParamsParamId::Timing34,
    PicaduraParamsParamId::Timing35,
    PicaduraParamsParamId::Timing36,
    PicaduraParamsParamId::Timing37,
    PicaduraParamsParamId::Timing38,
    PicaduraParamsParamId::Timing39,
    PicaduraParamsParamId::Timing40,
    PicaduraParamsParamId::Timing41,
    PicaduraParamsParamId::Timing42,
    PicaduraParamsParamId::Timing43,
    PicaduraParamsParamId::Timing44,
    PicaduraParamsParamId::Timing45,
    PicaduraParamsParamId::Timing46,
    PicaduraParamsParamId::Timing47,
    PicaduraParamsParamId::Timing48,
    PicaduraParamsParamId::Timing49,
    PicaduraParamsParamId::Timing50,
    PicaduraParamsParamId::Timing51,
    PicaduraParamsParamId::Timing52,
    PicaduraParamsParamId::Timing53,
    PicaduraParamsParamId::Timing54,
    PicaduraParamsParamId::Timing55,
    PicaduraParamsParamId::Timing56,
    PicaduraParamsParamId::Timing57,
    PicaduraParamsParamId::Timing58,
    PicaduraParamsParamId::Timing59,
    PicaduraParamsParamId::Timing60,
    PicaduraParamsParamId::Timing61,
    PicaduraParamsParamId::Timing62,
    PicaduraParamsParamId::Timing63,
];
