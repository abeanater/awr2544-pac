#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    info: Info,
    priirq: Priirq,
    prifiq: Prifiq,
    irqgsts: Irqgsts,
    fiqgsts: Fiqgsts,
    irqvec: Irqvec,
    fiqvec: Fiqvec,
    actirq: Actirq,
    actfiq: Actfiq,
    _reserved10: [u8; 0x08],
    dedvec: Dedvec,
    _reserved11: [u8; 0x03cc],
    raw: Raw,
    sts: Sts,
    intr_en_set: IntrEnSet,
    inter_en_clr: InterEnClr,
    irqsts: Irqsts,
    fiqsts: Fiqsts,
    intmap: Intmap,
    inttype: Inttype,
    raw_1: Raw1,
    sts_1: Sts1,
    intr_en_set_1: IntrEnSet1,
    inter_en_clr_1: InterEnClr1,
    irqsts_1: Irqsts1,
    fiqsts_1: Fiqsts1,
    intmap_1: Intmap1,
    inttype_1: Inttype1,
    raw_2: Raw2,
    sts_2: Sts2,
    intr_en_set_2: IntrEnSet2,
    inter_en_clr_2: InterEnClr2,
    irqsts_2: Irqsts2,
    fiqsts_2: Fiqsts2,
    intmap_2: Intmap2,
    inttype_2: Inttype2,
    raw_3: Raw3,
    sts_3: Sts3,
    intr_en_set_3: IntrEnSet3,
    inter_en_clr_3: InterEnClr3,
    irqsts_3: Irqsts3,
    fiqsts_3: Fiqsts3,
    intmap_3: Intmap3,
    inttype_3: Inttype3,
    raw_4: Raw4,
    sts_4: Sts4,
    intr_en_set_4: IntrEnSet4,
    inter_en_clr_4: InterEnClr4,
    irqsts_4: Irqsts4,
    fiqsts_4: Fiqsts4,
    intmap_4: Intmap4,
    inttype_4: Inttype4,
    raw_5: Raw5,
    sts_5: Sts5,
    intr_en_set_5: IntrEnSet5,
    inter_en_clr_5: InterEnClr5,
    irqsts_5: Irqsts5,
    fiqsts_5: Fiqsts5,
    intmap_5: Intmap5,
    inttype_5: Inttype5,
    raw_6: Raw6,
    sts_6: Sts6,
    intr_en_set_6: IntrEnSet6,
    inter_en_clr_6: InterEnClr6,
    irqsts_6: Irqsts6,
    fiqsts_6: Fiqsts6,
    intmap_6: Intmap6,
    inttype_6: Inttype6,
    raw_7: Raw7,
    sts_7: Sts7,
    intr_en_set_7: IntrEnSet7,
    inter_en_clr_7: InterEnClr7,
    irqsts_7: Irqsts7,
    fiqsts_7: Fiqsts7,
    intmap_7: Intmap7,
    inttype_7: Inttype7,
    _reserved75: [u8; 0x0b00],
    intpriority: Intpriority,
    intpriority_1: Intpriority1,
    intpriority_2: Intpriority2,
    intpriority_3: Intpriority3,
    intpriority_4: Intpriority4,
    intpriority_5: Intpriority5,
    intpriority_6: Intpriority6,
    intpriority_7: Intpriority7,
    intpriority_8: Intpriority8,
    intpriority_9: Intpriority9,
    intpriority_10: Intpriority10,
    intpriority_11: Intpriority11,
    intpriority_12: Intpriority12,
    intpriority_13: Intpriority13,
    intpriority_14: Intpriority14,
    intpriority_15: Intpriority15,
    intpriority_16: Intpriority16,
    intpriority_17: Intpriority17,
    intpriority_18: Intpriority18,
    intpriority_19: Intpriority19,
    intpriority_20: Intpriority20,
    intpriority_21: Intpriority21,
    intpriority_22: Intpriority22,
    intpriority_23: Intpriority23,
    intpriority_24: Intpriority24,
    intpriority_25: Intpriority25,
    intpriority_26: Intpriority26,
    intpriority_27: Intpriority27,
    intpriority_28: Intpriority28,
    intpriority_29: Intpriority29,
    intpriority_30: Intpriority30,
    intpriority_31: Intpriority31,
    intpriority_32: Intpriority32,
    intpriority_33: Intpriority33,
    intpriority_34: Intpriority34,
    intpriority_35: Intpriority35,
    intpriority_36: Intpriority36,
    intpriority_37: Intpriority37,
    intpriority_38: Intpriority38,
    intpriority_39: Intpriority39,
    intpriority_40: Intpriority40,
    intpriority_41: Intpriority41,
    intpriority_42: Intpriority42,
    intpriority_43: Intpriority43,
    intpriority_44: Intpriority44,
    intpriority_45: Intpriority45,
    intpriority_46: Intpriority46,
    intpriority_47: Intpriority47,
    intpriority_48: Intpriority48,
    intpriority_49: Intpriority49,
    intpriority_50: Intpriority50,
    intpriority_51: Intpriority51,
    intpriority_52: Intpriority52,
    intpriority_53: Intpriority53,
    intpriority_54: Intpriority54,
    intpriority_55: Intpriority55,
    intpriority_56: Intpriority56,
    intpriority_57: Intpriority57,
    intpriority_58: Intpriority58,
    intpriority_59: Intpriority59,
    intpriority_60: Intpriority60,
    intpriority_61: Intpriority61,
    intpriority_62: Intpriority62,
    intpriority_63: Intpriority63,
    intpriority_64: Intpriority64,
    intpriority_65: Intpriority65,
    intpriority_66: Intpriority66,
    intpriority_67: Intpriority67,
    intpriority_68: Intpriority68,
    intpriority_69: Intpriority69,
    intpriority_70: Intpriority70,
    intpriority_71: Intpriority71,
    intpriority_72: Intpriority72,
    intpriority_73: Intpriority73,
    intpriority_74: Intpriority74,
    intpriority_75: Intpriority75,
    intpriority_76: Intpriority76,
    intpriority_77: Intpriority77,
    intpriority_78: Intpriority78,
    intpriority_79: Intpriority79,
    intpriority_80: Intpriority80,
    intpriority_81: Intpriority81,
    intpriority_82: Intpriority82,
    intpriority_83: Intpriority83,
    intpriority_84: Intpriority84,
    intpriority_85: Intpriority85,
    intpriority_86: Intpriority86,
    intpriority_87: Intpriority87,
    intpriority_88: Intpriority88,
    intpriority_89: Intpriority89,
    intpriority_90: Intpriority90,
    intpriority_91: Intpriority91,
    intpriority_92: Intpriority92,
    intpriority_93: Intpriority93,
    intpriority_94: Intpriority94,
    intpriority_95: Intpriority95,
    intpriority_96: Intpriority96,
    intpriority_97: Intpriority97,
    intpriority_98: Intpriority98,
    intpriority_99: Intpriority99,
    intpriority_100: Intpriority100,
    intpriority_101: Intpriority101,
    intpriority_102: Intpriority102,
    intpriority_103: Intpriority103,
    intpriority_104: Intpriority104,
    intpriority_105: Intpriority105,
    intpriority_106: Intpriority106,
    intpriority_107: Intpriority107,
    intpriority_108: Intpriority108,
    intpriority_109: Intpriority109,
    intpriority_110: Intpriority110,
    intpriority_111: Intpriority111,
    intpriority_112: Intpriority112,
    intpriority_113: Intpriority113,
    intpriority_114: Intpriority114,
    intpriority_115: Intpriority115,
    intpriority_116: Intpriority116,
    intpriority_117: Intpriority117,
    intpriority_118: Intpriority118,
    intpriority_119: Intpriority119,
    intpriority_120: Intpriority120,
    intpriority_121: Intpriority121,
    intpriority_122: Intpriority122,
    intpriority_123: Intpriority123,
    intpriority_124: Intpriority124,
    intpriority_125: Intpriority125,
    intpriority_126: Intpriority126,
    intpriority_127: Intpriority127,
    intpriority_128: Intpriority128,
    intpriority_129: Intpriority129,
    intpriority_130: Intpriority130,
    intpriority_131: Intpriority131,
    intpriority_132: Intpriority132,
    intpriority_133: Intpriority133,
    intpriority_134: Intpriority134,
    intpriority_135: Intpriority135,
    intpriority_136: Intpriority136,
    intpriority_137: Intpriority137,
    intpriority_138: Intpriority138,
    intpriority_139: Intpriority139,
    intpriority_140: Intpriority140,
    intpriority_141: Intpriority141,
    intpriority_142: Intpriority142,
    intpriority_143: Intpriority143,
    intpriority_144: Intpriority144,
    intpriority_145: Intpriority145,
    intpriority_146: Intpriority146,
    intpriority_147: Intpriority147,
    intpriority_148: Intpriority148,
    intpriority_149: Intpriority149,
    intpriority_150: Intpriority150,
    intpriority_151: Intpriority151,
    intpriority_152: Intpriority152,
    intpriority_153: Intpriority153,
    intpriority_154: Intpriority154,
    intpriority_155: Intpriority155,
    intpriority_156: Intpriority156,
    intpriority_157: Intpriority157,
    intpriority_158: Intpriority158,
    intpriority_159: Intpriority159,
    intpriority_160: Intpriority160,
    intpriority_161: Intpriority161,
    intpriority_162: Intpriority162,
    intpriority_163: Intpriority163,
    intpriority_164: Intpriority164,
    intpriority_165: Intpriority165,
    intpriority_166: Intpriority166,
    intpriority_167: Intpriority167,
    intpriority_168: Intpriority168,
    intpriority_169: Intpriority169,
    intpriority_170: Intpriority170,
    intpriority_171: Intpriority171,
    intpriority_172: Intpriority172,
    intpriority_173: Intpriority173,
    intpriority_174: Intpriority174,
    intpriority_175: Intpriority175,
    intpriority_176: Intpriority176,
    intpriority_177: Intpriority177,
    intpriority_178: Intpriority178,
    intpriority_179: Intpriority179,
    intpriority_180: Intpriority180,
    intpriority_181: Intpriority181,
    intpriority_182: Intpriority182,
    intpriority_183: Intpriority183,
    intpriority_184: Intpriority184,
    intpriority_185: Intpriority185,
    intpriority_186: Intpriority186,
    intpriority_187: Intpriority187,
    intpriority_188: Intpriority188,
    intpriority_189: Intpriority189,
    intpriority_190: Intpriority190,
    intpriority_191: Intpriority191,
    intpriority_192: Intpriority192,
    intpriority_193: Intpriority193,
    intpriority_194: Intpriority194,
    intpriority_195: Intpriority195,
    intpriority_196: Intpriority196,
    intpriority_197: Intpriority197,
    intpriority_198: Intpriority198,
    intpriority_199: Intpriority199,
    intpriority_200: Intpriority200,
    intpriority_201: Intpriority201,
    intpriority_202: Intpriority202,
    intpriority_203: Intpriority203,
    intpriority_204: Intpriority204,
    intpriority_205: Intpriority205,
    intpriority_206: Intpriority206,
    intpriority_207: Intpriority207,
    intpriority_208: Intpriority208,
    intpriority_209: Intpriority209,
    intpriority_210: Intpriority210,
    intpriority_211: Intpriority211,
    intpriority_212: Intpriority212,
    intpriority_213: Intpriority213,
    intpriority_214: Intpriority214,
    intpriority_215: Intpriority215,
    intpriority_216: Intpriority216,
    intpriority_217: Intpriority217,
    intpriority_218: Intpriority218,
    intpriority_219: Intpriority219,
    intpriority_220: Intpriority220,
    intpriority_221: Intpriority221,
    intpriority_222: Intpriority222,
    intpriority_223: Intpriority223,
    intpriority_224: Intpriority224,
    intpriority_225: Intpriority225,
    intpriority_226: Intpriority226,
    intpriority_227: Intpriority227,
    intpriority_228: Intpriority228,
    intpriority_229: Intpriority229,
    intpriority_230: Intpriority230,
    intpriority_231: Intpriority231,
    intpriority_232: Intpriority232,
    intpriority_233: Intpriority233,
    intpriority_234: Intpriority234,
    intpriority_235: Intpriority235,
    intpriority_236: Intpriority236,
    intpriority_237: Intpriority237,
    intpriority_238: Intpriority238,
    intpriority_239: Intpriority239,
    intpriority_240: Intpriority240,
    intpriority_241: Intpriority241,
    intpriority_242: Intpriority242,
    intpriority_243: Intpriority243,
    intpriority_244: Intpriority244,
    intpriority_245: Intpriority245,
    intpriority_246: Intpriority246,
    intpriority_247: Intpriority247,
    intpriority_248: Intpriority248,
    intpriority_249: Intpriority249,
    intpriority_250: Intpriority250,
    intpriority_251: Intpriority251,
    intpriority_252: Intpriority252,
    intpriority_253: Intpriority253,
    intpriority_254: Intpriority254,
    intpriority_255: Intpriority255,
    _reserved331: [u8; 0x0c00],
    intvector: Intvector,
    intvector_1: Intvector1,
    intvector_2: Intvector2,
    intvector_3: Intvector3,
    intvector_4: Intvector4,
    intvector_5: Intvector5,
    intvector_6: Intvector6,
    intvector_7: Intvector7,
    intvector_8: Intvector8,
    intvector_9: Intvector9,
    intvector_10: Intvector10,
    intvector_11: Intvector11,
    intvector_12: Intvector12,
    intvector_13: Intvector13,
    intvector_14: Intvector14,
    intvector_15: Intvector15,
    intvector_16: Intvector16,
    intvector_17: Intvector17,
    intvector_18: Intvector18,
    intvector_19: Intvector19,
    intvector_20: Intvector20,
    intvector_21: Intvector21,
    intvector_22: Intvector22,
    intvector_23: Intvector23,
    intvector_24: Intvector24,
    intvector_25: Intvector25,
    intvector_26: Intvector26,
    intvector_27: Intvector27,
    intvector_28: Intvector28,
    intvector_29: Intvector29,
    intvector_30: Intvector30,
    intvector_31: Intvector31,
    intvector_32: Intvector32,
    intvector_33: Intvector33,
    intvector_34: Intvector34,
    intvector_35: Intvector35,
    intvector_36: Intvector36,
    intvector_37: Intvector37,
    intvector_38: Intvector38,
    intvector_39: Intvector39,
    intvector_40: Intvector40,
    intvector_41: Intvector41,
    intvector_42: Intvector42,
    intvector_43: Intvector43,
    intvector_44: Intvector44,
    intvector_45: Intvector45,
    intvector_46: Intvector46,
    intvector_47: Intvector47,
    intvector_48: Intvector48,
    intvector_49: Intvector49,
    intvector_50: Intvector50,
    intvector_51: Intvector51,
    intvector_52: Intvector52,
    intvector_53: Intvector53,
    intvector_54: Intvector54,
    intvector_55: Intvector55,
    intvector_56: Intvector56,
    intvector_57: Intvector57,
    intvector_58: Intvector58,
    intvector_59: Intvector59,
    intvector_60: Intvector60,
    intvector_61: Intvector61,
    intvector_62: Intvector62,
    intvector_63: Intvector63,
    intvector_64: Intvector64,
    intvector_65: Intvector65,
    intvector_66: Intvector66,
    intvector_67: Intvector67,
    intvector_68: Intvector68,
    intvector_69: Intvector69,
    intvector_70: Intvector70,
    intvector_71: Intvector71,
    intvector_72: Intvector72,
    intvector_73: Intvector73,
    intvector_74: Intvector74,
    intvector_75: Intvector75,
    intvector_76: Intvector76,
    intvector_77: Intvector77,
    intvector_78: Intvector78,
    intvector_79: Intvector79,
    intvector_80: Intvector80,
    intvector_81: Intvector81,
    intvector_82: Intvector82,
    intvector_83: Intvector83,
    intvector_84: Intvector84,
    intvector_85: Intvector85,
    intvector_86: Intvector86,
    intvector_87: Intvector87,
    intvector_88: Intvector88,
    intvector_89: Intvector89,
    intvector_90: Intvector90,
    intvector_91: Intvector91,
    intvector_92: Intvector92,
    intvector_93: Intvector93,
    intvector_94: Intvector94,
    intvector_95: Intvector95,
    intvector_96: Intvector96,
    intvector_97: Intvector97,
    intvector_98: Intvector98,
    intvector_99: Intvector99,
    intvector_100: Intvector100,
    intvector_101: Intvector101,
    intvector_102: Intvector102,
    intvector_103: Intvector103,
    intvector_104: Intvector104,
    intvector_105: Intvector105,
    intvector_106: Intvector106,
    intvector_107: Intvector107,
    intvector_108: Intvector108,
    intvector_109: Intvector109,
    intvector_110: Intvector110,
    intvector_111: Intvector111,
    intvector_112: Intvector112,
    intvector_113: Intvector113,
    intvector_114: Intvector114,
    intvector_115: Intvector115,
    intvector_116: Intvector116,
    intvector_117: Intvector117,
    intvector_118: Intvector118,
    intvector_119: Intvector119,
    intvector_120: Intvector120,
    intvector_121: Intvector121,
    intvector_122: Intvector122,
    intvector_123: Intvector123,
    intvector_124: Intvector124,
    intvector_125: Intvector125,
    intvector_126: Intvector126,
    intvector_127: Intvector127,
    intvector_128: Intvector128,
    intvector_129: Intvector129,
    intvector_130: Intvector130,
    intvector_131: Intvector131,
    intvector_132: Intvector132,
    intvector_133: Intvector133,
    intvector_134: Intvector134,
    intvector_135: Intvector135,
    intvector_136: Intvector136,
    intvector_137: Intvector137,
    intvector_138: Intvector138,
    intvector_139: Intvector139,
    intvector_140: Intvector140,
    intvector_141: Intvector141,
    intvector_142: Intvector142,
    intvector_143: Intvector143,
    intvector_144: Intvector144,
    intvector_145: Intvector145,
    intvector_146: Intvector146,
    intvector_147: Intvector147,
    intvector_148: Intvector148,
    intvector_149: Intvector149,
    intvector_150: Intvector150,
    intvector_151: Intvector151,
    intvector_152: Intvector152,
    intvector_153: Intvector153,
    intvector_154: Intvector154,
    intvector_155: Intvector155,
    intvector_156: Intvector156,
    intvector_157: Intvector157,
    intvector_158: Intvector158,
    intvector_159: Intvector159,
    intvector_160: Intvector160,
    intvector_161: Intvector161,
    intvector_162: Intvector162,
    intvector_163: Intvector163,
    intvector_164: Intvector164,
    intvector_165: Intvector165,
    intvector_166: Intvector166,
    intvector_167: Intvector167,
    intvector_168: Intvector168,
    intvector_169: Intvector169,
    intvector_170: Intvector170,
    intvector_171: Intvector171,
    intvector_172: Intvector172,
    intvector_173: Intvector173,
    intvector_174: Intvector174,
    intvector_175: Intvector175,
    intvector_176: Intvector176,
    intvector_177: Intvector177,
    intvector_178: Intvector178,
    intvector_179: Intvector179,
    intvector_180: Intvector180,
    intvector_181: Intvector181,
    intvector_182: Intvector182,
    intvector_183: Intvector183,
    intvector_184: Intvector184,
    intvector_185: Intvector185,
    intvector_186: Intvector186,
    intvector_187: Intvector187,
    intvector_188: Intvector188,
    intvector_189: Intvector189,
    intvector_190: Intvector190,
    intvector_191: Intvector191,
    intvector_192: Intvector192,
    intvector_193: Intvector193,
    intvector_194: Intvector194,
    intvector_195: Intvector195,
    intvector_196: Intvector196,
    intvector_197: Intvector197,
    intvector_198: Intvector198,
    intvector_199: Intvector199,
    intvector_200: Intvector200,
    intvector_201: Intvector201,
    intvector_202: Intvector202,
    intvector_203: Intvector203,
    intvector_204: Intvector204,
    intvector_205: Intvector205,
    intvector_206: Intvector206,
    intvector_207: Intvector207,
    intvector_208: Intvector208,
    intvector_209: Intvector209,
    intvector_210: Intvector210,
    intvector_211: Intvector211,
    intvector_212: Intvector212,
    intvector_213: Intvector213,
    intvector_214: Intvector214,
    intvector_215: Intvector215,
    intvector_216: Intvector216,
    intvector_217: Intvector217,
    intvector_218: Intvector218,
    intvector_219: Intvector219,
    intvector_220: Intvector220,
    intvector_221: Intvector221,
    intvector_222: Intvector222,
    intvector_223: Intvector223,
    intvector_224: Intvector224,
    intvector_225: Intvector225,
    intvector_226: Intvector226,
    intvector_227: Intvector227,
    intvector_228: Intvector228,
    intvector_229: Intvector229,
    intvector_230: Intvector230,
    intvector_231: Intvector231,
    intvector_232: Intvector232,
    intvector_233: Intvector233,
    intvector_234: Intvector234,
    intvector_235: Intvector235,
    intvector_236: Intvector236,
    intvector_237: Intvector237,
    intvector_238: Intvector238,
    intvector_239: Intvector239,
    intvector_240: Intvector240,
    intvector_241: Intvector241,
    intvector_242: Intvector242,
    intvector_243: Intvector243,
    intvector_244: Intvector244,
    intvector_245: Intvector245,
    intvector_246: Intvector246,
    intvector_247: Intvector247,
    intvector_248: Intvector248,
    intvector_249: Intvector249,
    intvector_250: Intvector250,
    intvector_251: Intvector251,
    intvector_252: Intvector252,
    intvector_253: Intvector253,
    intvector_254: Intvector254,
    intvector_255: Intvector255,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn pid(&self) -> &Pid {
        &self.pid
    }
    #[doc = "0x04 - The Info Register gives the configuration Inforrmation of this VIM."]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x08 - The Prioritized IRQ Register shows the number of the highest priority pending IRQ."]
    #[inline(always)]
    pub const fn priirq(&self) -> &Priirq {
        &self.priirq
    }
    #[doc = "0x0c - The Prioritized FIQ Register shows the number of the highest priority pending FIQ."]
    #[inline(always)]
    pub const fn prifiq(&self) -> &Prifiq {
        &self.prifiq
    }
    #[doc = "0x10 - The IRQ Group Status Register indicates which groups have pending IRQ interrupts."]
    #[inline(always)]
    pub const fn irqgsts(&self) -> &Irqgsts {
        &self.irqgsts
    }
    #[doc = "0x14 - The FIQ Group Status Register indicates which groups have pending FIQ interrupts."]
    #[inline(always)]
    pub const fn fiqgsts(&self) -> &Fiqgsts {
        &self.fiqgsts
    }
    #[doc = "0x18 - The IRQ Vector Address Register contains the 32-bit address of the interrupt vector for the current pendind IRQ."]
    #[inline(always)]
    pub const fn irqvec(&self) -> &Irqvec {
        &self.irqvec
    }
    #[doc = "0x1c - The FIQ Vector Address Register contains the 32-bit address of the interrupt vector for the current pendind FIQ."]
    #[inline(always)]
    pub const fn fiqvec(&self) -> &Fiqvec {
        &self.fiqvec
    }
    #[doc = "0x20 - The Active IRQ Register shows the number of the currently active IRQ."]
    #[inline(always)]
    pub const fn actirq(&self) -> &Actirq {
        &self.actirq
    }
    #[doc = "0x24 - The Active FIQ Register shows the number of the currently active FIQ."]
    #[inline(always)]
    pub const fn actfiq(&self) -> &Actfiq {
        &self.actfiq
    }
    #[doc = "0x30 - The DED Vector Address contains a default vector address for when an uncorrectable error is detected for an active IRQ or FIQ."]
    #[inline(always)]
    pub const fn dedvec(&self) -> &Dedvec {
        &self.dedvec
    }
    #[doc = "0x400 - Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
    #[inline(always)]
    pub const fn raw(&self) -> &Raw {
        &self.raw
    }
    #[doc = "0x404 - Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x408 - Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
    #[inline(always)]
    pub const fn intr_en_set(&self) -> &IntrEnSet {
        &self.intr_en_set
    }
    #[doc = "0x40c - Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
    #[inline(always)]
    pub const fn inter_en_clr(&self) -> &InterEnClr {
        &self.inter_en_clr
    }
    #[doc = "0x410 - Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
    #[inline(always)]
    pub const fn irqsts(&self) -> &Irqsts {
        &self.irqsts
    }
    #[doc = "0x414 - Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
    #[inline(always)]
    pub const fn fiqsts(&self) -> &Fiqsts {
        &self.fiqsts
    }
    #[doc = "0x418 - Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
    #[inline(always)]
    pub const fn intmap(&self) -> &Intmap {
        &self.intmap
    }
    #[doc = "0x41c - Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
    #[inline(always)]
    pub const fn inttype(&self) -> &Inttype {
        &self.inttype
    }
    #[doc = "0x420 - Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
    #[inline(always)]
    pub const fn raw_1(&self) -> &Raw1 {
        &self.raw_1
    }
    #[doc = "0x424 - Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
    #[inline(always)]
    pub const fn sts_1(&self) -> &Sts1 {
        &self.sts_1
    }
    #[doc = "0x428 - Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
    #[inline(always)]
    pub const fn intr_en_set_1(&self) -> &IntrEnSet1 {
        &self.intr_en_set_1
    }
    #[doc = "0x42c - Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
    #[inline(always)]
    pub const fn inter_en_clr_1(&self) -> &InterEnClr1 {
        &self.inter_en_clr_1
    }
    #[doc = "0x430 - Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
    #[inline(always)]
    pub const fn irqsts_1(&self) -> &Irqsts1 {
        &self.irqsts_1
    }
    #[doc = "0x434 - Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
    #[inline(always)]
    pub const fn fiqsts_1(&self) -> &Fiqsts1 {
        &self.fiqsts_1
    }
    #[doc = "0x438 - Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
    #[inline(always)]
    pub const fn intmap_1(&self) -> &Intmap1 {
        &self.intmap_1
    }
    #[doc = "0x43c - Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
    #[inline(always)]
    pub const fn inttype_1(&self) -> &Inttype1 {
        &self.inttype_1
    }
    #[doc = "0x440 - Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
    #[inline(always)]
    pub const fn raw_2(&self) -> &Raw2 {
        &self.raw_2
    }
    #[doc = "0x444 - Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
    #[inline(always)]
    pub const fn sts_2(&self) -> &Sts2 {
        &self.sts_2
    }
    #[doc = "0x448 - Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
    #[inline(always)]
    pub const fn intr_en_set_2(&self) -> &IntrEnSet2 {
        &self.intr_en_set_2
    }
    #[doc = "0x44c - Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
    #[inline(always)]
    pub const fn inter_en_clr_2(&self) -> &InterEnClr2 {
        &self.inter_en_clr_2
    }
    #[doc = "0x450 - Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
    #[inline(always)]
    pub const fn irqsts_2(&self) -> &Irqsts2 {
        &self.irqsts_2
    }
    #[doc = "0x454 - Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
    #[inline(always)]
    pub const fn fiqsts_2(&self) -> &Fiqsts2 {
        &self.fiqsts_2
    }
    #[doc = "0x458 - Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
    #[inline(always)]
    pub const fn intmap_2(&self) -> &Intmap2 {
        &self.intmap_2
    }
    #[doc = "0x45c - Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
    #[inline(always)]
    pub const fn inttype_2(&self) -> &Inttype2 {
        &self.inttype_2
    }
    #[doc = "0x460 - Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
    #[inline(always)]
    pub const fn raw_3(&self) -> &Raw3 {
        &self.raw_3
    }
    #[doc = "0x464 - Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
    #[inline(always)]
    pub const fn sts_3(&self) -> &Sts3 {
        &self.sts_3
    }
    #[doc = "0x468 - Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
    #[inline(always)]
    pub const fn intr_en_set_3(&self) -> &IntrEnSet3 {
        &self.intr_en_set_3
    }
    #[doc = "0x46c - Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
    #[inline(always)]
    pub const fn inter_en_clr_3(&self) -> &InterEnClr3 {
        &self.inter_en_clr_3
    }
    #[doc = "0x470 - Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
    #[inline(always)]
    pub const fn irqsts_3(&self) -> &Irqsts3 {
        &self.irqsts_3
    }
    #[doc = "0x474 - Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
    #[inline(always)]
    pub const fn fiqsts_3(&self) -> &Fiqsts3 {
        &self.fiqsts_3
    }
    #[doc = "0x478 - Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
    #[inline(always)]
    pub const fn intmap_3(&self) -> &Intmap3 {
        &self.intmap_3
    }
    #[doc = "0x47c - Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
    #[inline(always)]
    pub const fn inttype_3(&self) -> &Inttype3 {
        &self.inttype_3
    }
    #[doc = "0x480 - Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
    #[inline(always)]
    pub const fn raw_4(&self) -> &Raw4 {
        &self.raw_4
    }
    #[doc = "0x484 - Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
    #[inline(always)]
    pub const fn sts_4(&self) -> &Sts4 {
        &self.sts_4
    }
    #[doc = "0x488 - Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
    #[inline(always)]
    pub const fn intr_en_set_4(&self) -> &IntrEnSet4 {
        &self.intr_en_set_4
    }
    #[doc = "0x48c - Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
    #[inline(always)]
    pub const fn inter_en_clr_4(&self) -> &InterEnClr4 {
        &self.inter_en_clr_4
    }
    #[doc = "0x490 - Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
    #[inline(always)]
    pub const fn irqsts_4(&self) -> &Irqsts4 {
        &self.irqsts_4
    }
    #[doc = "0x494 - Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
    #[inline(always)]
    pub const fn fiqsts_4(&self) -> &Fiqsts4 {
        &self.fiqsts_4
    }
    #[doc = "0x498 - Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
    #[inline(always)]
    pub const fn intmap_4(&self) -> &Intmap4 {
        &self.intmap_4
    }
    #[doc = "0x49c - Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
    #[inline(always)]
    pub const fn inttype_4(&self) -> &Inttype4 {
        &self.inttype_4
    }
    #[doc = "0x4a0 - Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
    #[inline(always)]
    pub const fn raw_5(&self) -> &Raw5 {
        &self.raw_5
    }
    #[doc = "0x4a4 - Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
    #[inline(always)]
    pub const fn sts_5(&self) -> &Sts5 {
        &self.sts_5
    }
    #[doc = "0x4a8 - Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
    #[inline(always)]
    pub const fn intr_en_set_5(&self) -> &IntrEnSet5 {
        &self.intr_en_set_5
    }
    #[doc = "0x4ac - Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
    #[inline(always)]
    pub const fn inter_en_clr_5(&self) -> &InterEnClr5 {
        &self.inter_en_clr_5
    }
    #[doc = "0x4b0 - Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
    #[inline(always)]
    pub const fn irqsts_5(&self) -> &Irqsts5 {
        &self.irqsts_5
    }
    #[doc = "0x4b4 - Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
    #[inline(always)]
    pub const fn fiqsts_5(&self) -> &Fiqsts5 {
        &self.fiqsts_5
    }
    #[doc = "0x4b8 - Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
    #[inline(always)]
    pub const fn intmap_5(&self) -> &Intmap5 {
        &self.intmap_5
    }
    #[doc = "0x4bc - Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
    #[inline(always)]
    pub const fn inttype_5(&self) -> &Inttype5 {
        &self.inttype_5
    }
    #[doc = "0x4c0 - Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
    #[inline(always)]
    pub const fn raw_6(&self) -> &Raw6 {
        &self.raw_6
    }
    #[doc = "0x4c4 - Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
    #[inline(always)]
    pub const fn sts_6(&self) -> &Sts6 {
        &self.sts_6
    }
    #[doc = "0x4c8 - Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
    #[inline(always)]
    pub const fn intr_en_set_6(&self) -> &IntrEnSet6 {
        &self.intr_en_set_6
    }
    #[doc = "0x4cc - Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
    #[inline(always)]
    pub const fn inter_en_clr_6(&self) -> &InterEnClr6 {
        &self.inter_en_clr_6
    }
    #[doc = "0x4d0 - Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
    #[inline(always)]
    pub const fn irqsts_6(&self) -> &Irqsts6 {
        &self.irqsts_6
    }
    #[doc = "0x4d4 - Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
    #[inline(always)]
    pub const fn fiqsts_6(&self) -> &Fiqsts6 {
        &self.fiqsts_6
    }
    #[doc = "0x4d8 - Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
    #[inline(always)]
    pub const fn intmap_6(&self) -> &Intmap6 {
        &self.intmap_6
    }
    #[doc = "0x4dc - Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
    #[inline(always)]
    pub const fn inttype_6(&self) -> &Inttype6 {
        &self.inttype_6
    }
    #[doc = "0x4e0 - Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
    #[inline(always)]
    pub const fn raw_7(&self) -> &Raw7 {
        &self.raw_7
    }
    #[doc = "0x4e4 - Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
    #[inline(always)]
    pub const fn sts_7(&self) -> &Sts7 {
        &self.sts_7
    }
    #[doc = "0x4e8 - Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
    #[inline(always)]
    pub const fn intr_en_set_7(&self) -> &IntrEnSet7 {
        &self.intr_en_set_7
    }
    #[doc = "0x4ec - Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
    #[inline(always)]
    pub const fn inter_en_clr_7(&self) -> &InterEnClr7 {
        &self.inter_en_clr_7
    }
    #[doc = "0x4f0 - Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
    #[inline(always)]
    pub const fn irqsts_7(&self) -> &Irqsts7 {
        &self.irqsts_7
    }
    #[doc = "0x4f4 - Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
    #[inline(always)]
    pub const fn fiqsts_7(&self) -> &Fiqsts7 {
        &self.fiqsts_7
    }
    #[doc = "0x4f8 - Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
    #[inline(always)]
    pub const fn intmap_7(&self) -> &Intmap7 {
        &self.intmap_7
    }
    #[doc = "0x4fc - Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
    #[inline(always)]
    pub const fn inttype_7(&self) -> &Inttype7 {
        &self.inttype_7
    }
    #[doc = "0x1000 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h4"]
    #[inline(always)]
    pub const fn intpriority(&self) -> &Intpriority {
        &self.intpriority
    }
    #[doc = "0x1004 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h5"]
    #[inline(always)]
    pub const fn intpriority_1(&self) -> &Intpriority1 {
        &self.intpriority_1
    }
    #[doc = "0x1008 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h6"]
    #[inline(always)]
    pub const fn intpriority_2(&self) -> &Intpriority2 {
        &self.intpriority_2
    }
    #[doc = "0x100c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h7"]
    #[inline(always)]
    pub const fn intpriority_3(&self) -> &Intpriority3 {
        &self.intpriority_3
    }
    #[doc = "0x1010 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h8"]
    #[inline(always)]
    pub const fn intpriority_4(&self) -> &Intpriority4 {
        &self.intpriority_4
    }
    #[doc = "0x1014 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h9"]
    #[inline(always)]
    pub const fn intpriority_5(&self) -> &Intpriority5 {
        &self.intpriority_5
    }
    #[doc = "0x1018 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h10"]
    #[inline(always)]
    pub const fn intpriority_6(&self) -> &Intpriority6 {
        &self.intpriority_6
    }
    #[doc = "0x101c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h11"]
    #[inline(always)]
    pub const fn intpriority_7(&self) -> &Intpriority7 {
        &self.intpriority_7
    }
    #[doc = "0x1020 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h12"]
    #[inline(always)]
    pub const fn intpriority_8(&self) -> &Intpriority8 {
        &self.intpriority_8
    }
    #[doc = "0x1024 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h13"]
    #[inline(always)]
    pub const fn intpriority_9(&self) -> &Intpriority9 {
        &self.intpriority_9
    }
    #[doc = "0x1028 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h14"]
    #[inline(always)]
    pub const fn intpriority_10(&self) -> &Intpriority10 {
        &self.intpriority_10
    }
    #[doc = "0x102c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h15"]
    #[inline(always)]
    pub const fn intpriority_11(&self) -> &Intpriority11 {
        &self.intpriority_11
    }
    #[doc = "0x1030 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h16"]
    #[inline(always)]
    pub const fn intpriority_12(&self) -> &Intpriority12 {
        &self.intpriority_12
    }
    #[doc = "0x1034 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h17"]
    #[inline(always)]
    pub const fn intpriority_13(&self) -> &Intpriority13 {
        &self.intpriority_13
    }
    #[doc = "0x1038 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h18"]
    #[inline(always)]
    pub const fn intpriority_14(&self) -> &Intpriority14 {
        &self.intpriority_14
    }
    #[doc = "0x103c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h19"]
    #[inline(always)]
    pub const fn intpriority_15(&self) -> &Intpriority15 {
        &self.intpriority_15
    }
    #[doc = "0x1040 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h20"]
    #[inline(always)]
    pub const fn intpriority_16(&self) -> &Intpriority16 {
        &self.intpriority_16
    }
    #[doc = "0x1044 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h21"]
    #[inline(always)]
    pub const fn intpriority_17(&self) -> &Intpriority17 {
        &self.intpriority_17
    }
    #[doc = "0x1048 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h22"]
    #[inline(always)]
    pub const fn intpriority_18(&self) -> &Intpriority18 {
        &self.intpriority_18
    }
    #[doc = "0x104c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h23"]
    #[inline(always)]
    pub const fn intpriority_19(&self) -> &Intpriority19 {
        &self.intpriority_19
    }
    #[doc = "0x1050 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h24"]
    #[inline(always)]
    pub const fn intpriority_20(&self) -> &Intpriority20 {
        &self.intpriority_20
    }
    #[doc = "0x1054 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h25"]
    #[inline(always)]
    pub const fn intpriority_21(&self) -> &Intpriority21 {
        &self.intpriority_21
    }
    #[doc = "0x1058 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h26"]
    #[inline(always)]
    pub const fn intpriority_22(&self) -> &Intpriority22 {
        &self.intpriority_22
    }
    #[doc = "0x105c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h27"]
    #[inline(always)]
    pub const fn intpriority_23(&self) -> &Intpriority23 {
        &self.intpriority_23
    }
    #[doc = "0x1060 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h28"]
    #[inline(always)]
    pub const fn intpriority_24(&self) -> &Intpriority24 {
        &self.intpriority_24
    }
    #[doc = "0x1064 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h29"]
    #[inline(always)]
    pub const fn intpriority_25(&self) -> &Intpriority25 {
        &self.intpriority_25
    }
    #[doc = "0x1068 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h30"]
    #[inline(always)]
    pub const fn intpriority_26(&self) -> &Intpriority26 {
        &self.intpriority_26
    }
    #[doc = "0x106c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h31"]
    #[inline(always)]
    pub const fn intpriority_27(&self) -> &Intpriority27 {
        &self.intpriority_27
    }
    #[doc = "0x1070 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h32"]
    #[inline(always)]
    pub const fn intpriority_28(&self) -> &Intpriority28 {
        &self.intpriority_28
    }
    #[doc = "0x1074 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h33"]
    #[inline(always)]
    pub const fn intpriority_29(&self) -> &Intpriority29 {
        &self.intpriority_29
    }
    #[doc = "0x1078 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h34"]
    #[inline(always)]
    pub const fn intpriority_30(&self) -> &Intpriority30 {
        &self.intpriority_30
    }
    #[doc = "0x107c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h35"]
    #[inline(always)]
    pub const fn intpriority_31(&self) -> &Intpriority31 {
        &self.intpriority_31
    }
    #[doc = "0x1080 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h36"]
    #[inline(always)]
    pub const fn intpriority_32(&self) -> &Intpriority32 {
        &self.intpriority_32
    }
    #[doc = "0x1084 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h37"]
    #[inline(always)]
    pub const fn intpriority_33(&self) -> &Intpriority33 {
        &self.intpriority_33
    }
    #[doc = "0x1088 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h38"]
    #[inline(always)]
    pub const fn intpriority_34(&self) -> &Intpriority34 {
        &self.intpriority_34
    }
    #[doc = "0x108c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h39"]
    #[inline(always)]
    pub const fn intpriority_35(&self) -> &Intpriority35 {
        &self.intpriority_35
    }
    #[doc = "0x1090 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h40"]
    #[inline(always)]
    pub const fn intpriority_36(&self) -> &Intpriority36 {
        &self.intpriority_36
    }
    #[doc = "0x1094 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h41"]
    #[inline(always)]
    pub const fn intpriority_37(&self) -> &Intpriority37 {
        &self.intpriority_37
    }
    #[doc = "0x1098 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h42"]
    #[inline(always)]
    pub const fn intpriority_38(&self) -> &Intpriority38 {
        &self.intpriority_38
    }
    #[doc = "0x109c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h43"]
    #[inline(always)]
    pub const fn intpriority_39(&self) -> &Intpriority39 {
        &self.intpriority_39
    }
    #[doc = "0x10a0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h44"]
    #[inline(always)]
    pub const fn intpriority_40(&self) -> &Intpriority40 {
        &self.intpriority_40
    }
    #[doc = "0x10a4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h45"]
    #[inline(always)]
    pub const fn intpriority_41(&self) -> &Intpriority41 {
        &self.intpriority_41
    }
    #[doc = "0x10a8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h46"]
    #[inline(always)]
    pub const fn intpriority_42(&self) -> &Intpriority42 {
        &self.intpriority_42
    }
    #[doc = "0x10ac - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h47"]
    #[inline(always)]
    pub const fn intpriority_43(&self) -> &Intpriority43 {
        &self.intpriority_43
    }
    #[doc = "0x10b0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h48"]
    #[inline(always)]
    pub const fn intpriority_44(&self) -> &Intpriority44 {
        &self.intpriority_44
    }
    #[doc = "0x10b4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h49"]
    #[inline(always)]
    pub const fn intpriority_45(&self) -> &Intpriority45 {
        &self.intpriority_45
    }
    #[doc = "0x10b8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h50"]
    #[inline(always)]
    pub const fn intpriority_46(&self) -> &Intpriority46 {
        &self.intpriority_46
    }
    #[doc = "0x10bc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h51"]
    #[inline(always)]
    pub const fn intpriority_47(&self) -> &Intpriority47 {
        &self.intpriority_47
    }
    #[doc = "0x10c0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h52"]
    #[inline(always)]
    pub const fn intpriority_48(&self) -> &Intpriority48 {
        &self.intpriority_48
    }
    #[doc = "0x10c4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h53"]
    #[inline(always)]
    pub const fn intpriority_49(&self) -> &Intpriority49 {
        &self.intpriority_49
    }
    #[doc = "0x10c8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h54"]
    #[inline(always)]
    pub const fn intpriority_50(&self) -> &Intpriority50 {
        &self.intpriority_50
    }
    #[doc = "0x10cc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h55"]
    #[inline(always)]
    pub const fn intpriority_51(&self) -> &Intpriority51 {
        &self.intpriority_51
    }
    #[doc = "0x10d0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h56"]
    #[inline(always)]
    pub const fn intpriority_52(&self) -> &Intpriority52 {
        &self.intpriority_52
    }
    #[doc = "0x10d4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h57"]
    #[inline(always)]
    pub const fn intpriority_53(&self) -> &Intpriority53 {
        &self.intpriority_53
    }
    #[doc = "0x10d8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h58"]
    #[inline(always)]
    pub const fn intpriority_54(&self) -> &Intpriority54 {
        &self.intpriority_54
    }
    #[doc = "0x10dc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h59"]
    #[inline(always)]
    pub const fn intpriority_55(&self) -> &Intpriority55 {
        &self.intpriority_55
    }
    #[doc = "0x10e0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h60"]
    #[inline(always)]
    pub const fn intpriority_56(&self) -> &Intpriority56 {
        &self.intpriority_56
    }
    #[doc = "0x10e4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h61"]
    #[inline(always)]
    pub const fn intpriority_57(&self) -> &Intpriority57 {
        &self.intpriority_57
    }
    #[doc = "0x10e8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h62"]
    #[inline(always)]
    pub const fn intpriority_58(&self) -> &Intpriority58 {
        &self.intpriority_58
    }
    #[doc = "0x10ec - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h63"]
    #[inline(always)]
    pub const fn intpriority_59(&self) -> &Intpriority59 {
        &self.intpriority_59
    }
    #[doc = "0x10f0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h64"]
    #[inline(always)]
    pub const fn intpriority_60(&self) -> &Intpriority60 {
        &self.intpriority_60
    }
    #[doc = "0x10f4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h65"]
    #[inline(always)]
    pub const fn intpriority_61(&self) -> &Intpriority61 {
        &self.intpriority_61
    }
    #[doc = "0x10f8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h66"]
    #[inline(always)]
    pub const fn intpriority_62(&self) -> &Intpriority62 {
        &self.intpriority_62
    }
    #[doc = "0x10fc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h67"]
    #[inline(always)]
    pub const fn intpriority_63(&self) -> &Intpriority63 {
        &self.intpriority_63
    }
    #[doc = "0x1100 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h68"]
    #[inline(always)]
    pub const fn intpriority_64(&self) -> &Intpriority64 {
        &self.intpriority_64
    }
    #[doc = "0x1104 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h69"]
    #[inline(always)]
    pub const fn intpriority_65(&self) -> &Intpriority65 {
        &self.intpriority_65
    }
    #[doc = "0x1108 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h70"]
    #[inline(always)]
    pub const fn intpriority_66(&self) -> &Intpriority66 {
        &self.intpriority_66
    }
    #[doc = "0x110c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h71"]
    #[inline(always)]
    pub const fn intpriority_67(&self) -> &Intpriority67 {
        &self.intpriority_67
    }
    #[doc = "0x1110 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h72"]
    #[inline(always)]
    pub const fn intpriority_68(&self) -> &Intpriority68 {
        &self.intpriority_68
    }
    #[doc = "0x1114 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h73"]
    #[inline(always)]
    pub const fn intpriority_69(&self) -> &Intpriority69 {
        &self.intpriority_69
    }
    #[doc = "0x1118 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h74"]
    #[inline(always)]
    pub const fn intpriority_70(&self) -> &Intpriority70 {
        &self.intpriority_70
    }
    #[doc = "0x111c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h75"]
    #[inline(always)]
    pub const fn intpriority_71(&self) -> &Intpriority71 {
        &self.intpriority_71
    }
    #[doc = "0x1120 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h76"]
    #[inline(always)]
    pub const fn intpriority_72(&self) -> &Intpriority72 {
        &self.intpriority_72
    }
    #[doc = "0x1124 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h77"]
    #[inline(always)]
    pub const fn intpriority_73(&self) -> &Intpriority73 {
        &self.intpriority_73
    }
    #[doc = "0x1128 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h78"]
    #[inline(always)]
    pub const fn intpriority_74(&self) -> &Intpriority74 {
        &self.intpriority_74
    }
    #[doc = "0x112c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h79"]
    #[inline(always)]
    pub const fn intpriority_75(&self) -> &Intpriority75 {
        &self.intpriority_75
    }
    #[doc = "0x1130 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h80"]
    #[inline(always)]
    pub const fn intpriority_76(&self) -> &Intpriority76 {
        &self.intpriority_76
    }
    #[doc = "0x1134 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h81"]
    #[inline(always)]
    pub const fn intpriority_77(&self) -> &Intpriority77 {
        &self.intpriority_77
    }
    #[doc = "0x1138 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h82"]
    #[inline(always)]
    pub const fn intpriority_78(&self) -> &Intpriority78 {
        &self.intpriority_78
    }
    #[doc = "0x113c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h83"]
    #[inline(always)]
    pub const fn intpriority_79(&self) -> &Intpriority79 {
        &self.intpriority_79
    }
    #[doc = "0x1140 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h84"]
    #[inline(always)]
    pub const fn intpriority_80(&self) -> &Intpriority80 {
        &self.intpriority_80
    }
    #[doc = "0x1144 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h85"]
    #[inline(always)]
    pub const fn intpriority_81(&self) -> &Intpriority81 {
        &self.intpriority_81
    }
    #[doc = "0x1148 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h86"]
    #[inline(always)]
    pub const fn intpriority_82(&self) -> &Intpriority82 {
        &self.intpriority_82
    }
    #[doc = "0x114c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h87"]
    #[inline(always)]
    pub const fn intpriority_83(&self) -> &Intpriority83 {
        &self.intpriority_83
    }
    #[doc = "0x1150 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h88"]
    #[inline(always)]
    pub const fn intpriority_84(&self) -> &Intpriority84 {
        &self.intpriority_84
    }
    #[doc = "0x1154 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h89"]
    #[inline(always)]
    pub const fn intpriority_85(&self) -> &Intpriority85 {
        &self.intpriority_85
    }
    #[doc = "0x1158 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h90"]
    #[inline(always)]
    pub const fn intpriority_86(&self) -> &Intpriority86 {
        &self.intpriority_86
    }
    #[doc = "0x115c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h91"]
    #[inline(always)]
    pub const fn intpriority_87(&self) -> &Intpriority87 {
        &self.intpriority_87
    }
    #[doc = "0x1160 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h92"]
    #[inline(always)]
    pub const fn intpriority_88(&self) -> &Intpriority88 {
        &self.intpriority_88
    }
    #[doc = "0x1164 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h93"]
    #[inline(always)]
    pub const fn intpriority_89(&self) -> &Intpriority89 {
        &self.intpriority_89
    }
    #[doc = "0x1168 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h94"]
    #[inline(always)]
    pub const fn intpriority_90(&self) -> &Intpriority90 {
        &self.intpriority_90
    }
    #[doc = "0x116c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h95"]
    #[inline(always)]
    pub const fn intpriority_91(&self) -> &Intpriority91 {
        &self.intpriority_91
    }
    #[doc = "0x1170 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h96"]
    #[inline(always)]
    pub const fn intpriority_92(&self) -> &Intpriority92 {
        &self.intpriority_92
    }
    #[doc = "0x1174 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h97"]
    #[inline(always)]
    pub const fn intpriority_93(&self) -> &Intpriority93 {
        &self.intpriority_93
    }
    #[doc = "0x1178 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h98"]
    #[inline(always)]
    pub const fn intpriority_94(&self) -> &Intpriority94 {
        &self.intpriority_94
    }
    #[doc = "0x117c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h99"]
    #[inline(always)]
    pub const fn intpriority_95(&self) -> &Intpriority95 {
        &self.intpriority_95
    }
    #[doc = "0x1180 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h100"]
    #[inline(always)]
    pub const fn intpriority_96(&self) -> &Intpriority96 {
        &self.intpriority_96
    }
    #[doc = "0x1184 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h101"]
    #[inline(always)]
    pub const fn intpriority_97(&self) -> &Intpriority97 {
        &self.intpriority_97
    }
    #[doc = "0x1188 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h102"]
    #[inline(always)]
    pub const fn intpriority_98(&self) -> &Intpriority98 {
        &self.intpriority_98
    }
    #[doc = "0x118c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h103"]
    #[inline(always)]
    pub const fn intpriority_99(&self) -> &Intpriority99 {
        &self.intpriority_99
    }
    #[doc = "0x1190 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h104"]
    #[inline(always)]
    pub const fn intpriority_100(&self) -> &Intpriority100 {
        &self.intpriority_100
    }
    #[doc = "0x1194 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h105"]
    #[inline(always)]
    pub const fn intpriority_101(&self) -> &Intpriority101 {
        &self.intpriority_101
    }
    #[doc = "0x1198 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h106"]
    #[inline(always)]
    pub const fn intpriority_102(&self) -> &Intpriority102 {
        &self.intpriority_102
    }
    #[doc = "0x119c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h107"]
    #[inline(always)]
    pub const fn intpriority_103(&self) -> &Intpriority103 {
        &self.intpriority_103
    }
    #[doc = "0x11a0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h108"]
    #[inline(always)]
    pub const fn intpriority_104(&self) -> &Intpriority104 {
        &self.intpriority_104
    }
    #[doc = "0x11a4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h109"]
    #[inline(always)]
    pub const fn intpriority_105(&self) -> &Intpriority105 {
        &self.intpriority_105
    }
    #[doc = "0x11a8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h110"]
    #[inline(always)]
    pub const fn intpriority_106(&self) -> &Intpriority106 {
        &self.intpriority_106
    }
    #[doc = "0x11ac - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h111"]
    #[inline(always)]
    pub const fn intpriority_107(&self) -> &Intpriority107 {
        &self.intpriority_107
    }
    #[doc = "0x11b0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h112"]
    #[inline(always)]
    pub const fn intpriority_108(&self) -> &Intpriority108 {
        &self.intpriority_108
    }
    #[doc = "0x11b4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h113"]
    #[inline(always)]
    pub const fn intpriority_109(&self) -> &Intpriority109 {
        &self.intpriority_109
    }
    #[doc = "0x11b8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h114"]
    #[inline(always)]
    pub const fn intpriority_110(&self) -> &Intpriority110 {
        &self.intpriority_110
    }
    #[doc = "0x11bc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h115"]
    #[inline(always)]
    pub const fn intpriority_111(&self) -> &Intpriority111 {
        &self.intpriority_111
    }
    #[doc = "0x11c0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h116"]
    #[inline(always)]
    pub const fn intpriority_112(&self) -> &Intpriority112 {
        &self.intpriority_112
    }
    #[doc = "0x11c4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h117"]
    #[inline(always)]
    pub const fn intpriority_113(&self) -> &Intpriority113 {
        &self.intpriority_113
    }
    #[doc = "0x11c8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h118"]
    #[inline(always)]
    pub const fn intpriority_114(&self) -> &Intpriority114 {
        &self.intpriority_114
    }
    #[doc = "0x11cc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h119"]
    #[inline(always)]
    pub const fn intpriority_115(&self) -> &Intpriority115 {
        &self.intpriority_115
    }
    #[doc = "0x11d0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h120"]
    #[inline(always)]
    pub const fn intpriority_116(&self) -> &Intpriority116 {
        &self.intpriority_116
    }
    #[doc = "0x11d4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h121"]
    #[inline(always)]
    pub const fn intpriority_117(&self) -> &Intpriority117 {
        &self.intpriority_117
    }
    #[doc = "0x11d8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h122"]
    #[inline(always)]
    pub const fn intpriority_118(&self) -> &Intpriority118 {
        &self.intpriority_118
    }
    #[doc = "0x11dc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h123"]
    #[inline(always)]
    pub const fn intpriority_119(&self) -> &Intpriority119 {
        &self.intpriority_119
    }
    #[doc = "0x11e0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h124"]
    #[inline(always)]
    pub const fn intpriority_120(&self) -> &Intpriority120 {
        &self.intpriority_120
    }
    #[doc = "0x11e4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h125"]
    #[inline(always)]
    pub const fn intpriority_121(&self) -> &Intpriority121 {
        &self.intpriority_121
    }
    #[doc = "0x11e8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h126"]
    #[inline(always)]
    pub const fn intpriority_122(&self) -> &Intpriority122 {
        &self.intpriority_122
    }
    #[doc = "0x11ec - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h127"]
    #[inline(always)]
    pub const fn intpriority_123(&self) -> &Intpriority123 {
        &self.intpriority_123
    }
    #[doc = "0x11f0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h128"]
    #[inline(always)]
    pub const fn intpriority_124(&self) -> &Intpriority124 {
        &self.intpriority_124
    }
    #[doc = "0x11f4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h129"]
    #[inline(always)]
    pub const fn intpriority_125(&self) -> &Intpriority125 {
        &self.intpriority_125
    }
    #[doc = "0x11f8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h130"]
    #[inline(always)]
    pub const fn intpriority_126(&self) -> &Intpriority126 {
        &self.intpriority_126
    }
    #[doc = "0x11fc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h131"]
    #[inline(always)]
    pub const fn intpriority_127(&self) -> &Intpriority127 {
        &self.intpriority_127
    }
    #[doc = "0x1200 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h132"]
    #[inline(always)]
    pub const fn intpriority_128(&self) -> &Intpriority128 {
        &self.intpriority_128
    }
    #[doc = "0x1204 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h133"]
    #[inline(always)]
    pub const fn intpriority_129(&self) -> &Intpriority129 {
        &self.intpriority_129
    }
    #[doc = "0x1208 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h134"]
    #[inline(always)]
    pub const fn intpriority_130(&self) -> &Intpriority130 {
        &self.intpriority_130
    }
    #[doc = "0x120c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h135"]
    #[inline(always)]
    pub const fn intpriority_131(&self) -> &Intpriority131 {
        &self.intpriority_131
    }
    #[doc = "0x1210 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h136"]
    #[inline(always)]
    pub const fn intpriority_132(&self) -> &Intpriority132 {
        &self.intpriority_132
    }
    #[doc = "0x1214 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h137"]
    #[inline(always)]
    pub const fn intpriority_133(&self) -> &Intpriority133 {
        &self.intpriority_133
    }
    #[doc = "0x1218 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h138"]
    #[inline(always)]
    pub const fn intpriority_134(&self) -> &Intpriority134 {
        &self.intpriority_134
    }
    #[doc = "0x121c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h139"]
    #[inline(always)]
    pub const fn intpriority_135(&self) -> &Intpriority135 {
        &self.intpriority_135
    }
    #[doc = "0x1220 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h140"]
    #[inline(always)]
    pub const fn intpriority_136(&self) -> &Intpriority136 {
        &self.intpriority_136
    }
    #[doc = "0x1224 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h141"]
    #[inline(always)]
    pub const fn intpriority_137(&self) -> &Intpriority137 {
        &self.intpriority_137
    }
    #[doc = "0x1228 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h142"]
    #[inline(always)]
    pub const fn intpriority_138(&self) -> &Intpriority138 {
        &self.intpriority_138
    }
    #[doc = "0x122c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h143"]
    #[inline(always)]
    pub const fn intpriority_139(&self) -> &Intpriority139 {
        &self.intpriority_139
    }
    #[doc = "0x1230 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h144"]
    #[inline(always)]
    pub const fn intpriority_140(&self) -> &Intpriority140 {
        &self.intpriority_140
    }
    #[doc = "0x1234 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h145"]
    #[inline(always)]
    pub const fn intpriority_141(&self) -> &Intpriority141 {
        &self.intpriority_141
    }
    #[doc = "0x1238 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h146"]
    #[inline(always)]
    pub const fn intpriority_142(&self) -> &Intpriority142 {
        &self.intpriority_142
    }
    #[doc = "0x123c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h147"]
    #[inline(always)]
    pub const fn intpriority_143(&self) -> &Intpriority143 {
        &self.intpriority_143
    }
    #[doc = "0x1240 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h148"]
    #[inline(always)]
    pub const fn intpriority_144(&self) -> &Intpriority144 {
        &self.intpriority_144
    }
    #[doc = "0x1244 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h149"]
    #[inline(always)]
    pub const fn intpriority_145(&self) -> &Intpriority145 {
        &self.intpriority_145
    }
    #[doc = "0x1248 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h150"]
    #[inline(always)]
    pub const fn intpriority_146(&self) -> &Intpriority146 {
        &self.intpriority_146
    }
    #[doc = "0x124c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h151"]
    #[inline(always)]
    pub const fn intpriority_147(&self) -> &Intpriority147 {
        &self.intpriority_147
    }
    #[doc = "0x1250 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h152"]
    #[inline(always)]
    pub const fn intpriority_148(&self) -> &Intpriority148 {
        &self.intpriority_148
    }
    #[doc = "0x1254 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h153"]
    #[inline(always)]
    pub const fn intpriority_149(&self) -> &Intpriority149 {
        &self.intpriority_149
    }
    #[doc = "0x1258 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h154"]
    #[inline(always)]
    pub const fn intpriority_150(&self) -> &Intpriority150 {
        &self.intpriority_150
    }
    #[doc = "0x125c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h155"]
    #[inline(always)]
    pub const fn intpriority_151(&self) -> &Intpriority151 {
        &self.intpriority_151
    }
    #[doc = "0x1260 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h156"]
    #[inline(always)]
    pub const fn intpriority_152(&self) -> &Intpriority152 {
        &self.intpriority_152
    }
    #[doc = "0x1264 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h157"]
    #[inline(always)]
    pub const fn intpriority_153(&self) -> &Intpriority153 {
        &self.intpriority_153
    }
    #[doc = "0x1268 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h158"]
    #[inline(always)]
    pub const fn intpriority_154(&self) -> &Intpriority154 {
        &self.intpriority_154
    }
    #[doc = "0x126c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h159"]
    #[inline(always)]
    pub const fn intpriority_155(&self) -> &Intpriority155 {
        &self.intpriority_155
    }
    #[doc = "0x1270 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h160"]
    #[inline(always)]
    pub const fn intpriority_156(&self) -> &Intpriority156 {
        &self.intpriority_156
    }
    #[doc = "0x1274 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h161"]
    #[inline(always)]
    pub const fn intpriority_157(&self) -> &Intpriority157 {
        &self.intpriority_157
    }
    #[doc = "0x1278 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h162"]
    #[inline(always)]
    pub const fn intpriority_158(&self) -> &Intpriority158 {
        &self.intpriority_158
    }
    #[doc = "0x127c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h163"]
    #[inline(always)]
    pub const fn intpriority_159(&self) -> &Intpriority159 {
        &self.intpriority_159
    }
    #[doc = "0x1280 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h164"]
    #[inline(always)]
    pub const fn intpriority_160(&self) -> &Intpriority160 {
        &self.intpriority_160
    }
    #[doc = "0x1284 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h165"]
    #[inline(always)]
    pub const fn intpriority_161(&self) -> &Intpriority161 {
        &self.intpriority_161
    }
    #[doc = "0x1288 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h166"]
    #[inline(always)]
    pub const fn intpriority_162(&self) -> &Intpriority162 {
        &self.intpriority_162
    }
    #[doc = "0x128c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h167"]
    #[inline(always)]
    pub const fn intpriority_163(&self) -> &Intpriority163 {
        &self.intpriority_163
    }
    #[doc = "0x1290 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h168"]
    #[inline(always)]
    pub const fn intpriority_164(&self) -> &Intpriority164 {
        &self.intpriority_164
    }
    #[doc = "0x1294 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h169"]
    #[inline(always)]
    pub const fn intpriority_165(&self) -> &Intpriority165 {
        &self.intpriority_165
    }
    #[doc = "0x1298 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h170"]
    #[inline(always)]
    pub const fn intpriority_166(&self) -> &Intpriority166 {
        &self.intpriority_166
    }
    #[doc = "0x129c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h171"]
    #[inline(always)]
    pub const fn intpriority_167(&self) -> &Intpriority167 {
        &self.intpriority_167
    }
    #[doc = "0x12a0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h172"]
    #[inline(always)]
    pub const fn intpriority_168(&self) -> &Intpriority168 {
        &self.intpriority_168
    }
    #[doc = "0x12a4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h173"]
    #[inline(always)]
    pub const fn intpriority_169(&self) -> &Intpriority169 {
        &self.intpriority_169
    }
    #[doc = "0x12a8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h174"]
    #[inline(always)]
    pub const fn intpriority_170(&self) -> &Intpriority170 {
        &self.intpriority_170
    }
    #[doc = "0x12ac - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h175"]
    #[inline(always)]
    pub const fn intpriority_171(&self) -> &Intpriority171 {
        &self.intpriority_171
    }
    #[doc = "0x12b0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h176"]
    #[inline(always)]
    pub const fn intpriority_172(&self) -> &Intpriority172 {
        &self.intpriority_172
    }
    #[doc = "0x12b4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h177"]
    #[inline(always)]
    pub const fn intpriority_173(&self) -> &Intpriority173 {
        &self.intpriority_173
    }
    #[doc = "0x12b8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h178"]
    #[inline(always)]
    pub const fn intpriority_174(&self) -> &Intpriority174 {
        &self.intpriority_174
    }
    #[doc = "0x12bc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h179"]
    #[inline(always)]
    pub const fn intpriority_175(&self) -> &Intpriority175 {
        &self.intpriority_175
    }
    #[doc = "0x12c0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h180"]
    #[inline(always)]
    pub const fn intpriority_176(&self) -> &Intpriority176 {
        &self.intpriority_176
    }
    #[doc = "0x12c4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h181"]
    #[inline(always)]
    pub const fn intpriority_177(&self) -> &Intpriority177 {
        &self.intpriority_177
    }
    #[doc = "0x12c8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h182"]
    #[inline(always)]
    pub const fn intpriority_178(&self) -> &Intpriority178 {
        &self.intpriority_178
    }
    #[doc = "0x12cc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h183"]
    #[inline(always)]
    pub const fn intpriority_179(&self) -> &Intpriority179 {
        &self.intpriority_179
    }
    #[doc = "0x12d0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h184"]
    #[inline(always)]
    pub const fn intpriority_180(&self) -> &Intpriority180 {
        &self.intpriority_180
    }
    #[doc = "0x12d4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h185"]
    #[inline(always)]
    pub const fn intpriority_181(&self) -> &Intpriority181 {
        &self.intpriority_181
    }
    #[doc = "0x12d8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h186"]
    #[inline(always)]
    pub const fn intpriority_182(&self) -> &Intpriority182 {
        &self.intpriority_182
    }
    #[doc = "0x12dc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h187"]
    #[inline(always)]
    pub const fn intpriority_183(&self) -> &Intpriority183 {
        &self.intpriority_183
    }
    #[doc = "0x12e0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h188"]
    #[inline(always)]
    pub const fn intpriority_184(&self) -> &Intpriority184 {
        &self.intpriority_184
    }
    #[doc = "0x12e4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h189"]
    #[inline(always)]
    pub const fn intpriority_185(&self) -> &Intpriority185 {
        &self.intpriority_185
    }
    #[doc = "0x12e8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h190"]
    #[inline(always)]
    pub const fn intpriority_186(&self) -> &Intpriority186 {
        &self.intpriority_186
    }
    #[doc = "0x12ec - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h191"]
    #[inline(always)]
    pub const fn intpriority_187(&self) -> &Intpriority187 {
        &self.intpriority_187
    }
    #[doc = "0x12f0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h192"]
    #[inline(always)]
    pub const fn intpriority_188(&self) -> &Intpriority188 {
        &self.intpriority_188
    }
    #[doc = "0x12f4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h193"]
    #[inline(always)]
    pub const fn intpriority_189(&self) -> &Intpriority189 {
        &self.intpriority_189
    }
    #[doc = "0x12f8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h194"]
    #[inline(always)]
    pub const fn intpriority_190(&self) -> &Intpriority190 {
        &self.intpriority_190
    }
    #[doc = "0x12fc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h195"]
    #[inline(always)]
    pub const fn intpriority_191(&self) -> &Intpriority191 {
        &self.intpriority_191
    }
    #[doc = "0x1300 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h196"]
    #[inline(always)]
    pub const fn intpriority_192(&self) -> &Intpriority192 {
        &self.intpriority_192
    }
    #[doc = "0x1304 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h197"]
    #[inline(always)]
    pub const fn intpriority_193(&self) -> &Intpriority193 {
        &self.intpriority_193
    }
    #[doc = "0x1308 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h198"]
    #[inline(always)]
    pub const fn intpriority_194(&self) -> &Intpriority194 {
        &self.intpriority_194
    }
    #[doc = "0x130c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h199"]
    #[inline(always)]
    pub const fn intpriority_195(&self) -> &Intpriority195 {
        &self.intpriority_195
    }
    #[doc = "0x1310 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h200"]
    #[inline(always)]
    pub const fn intpriority_196(&self) -> &Intpriority196 {
        &self.intpriority_196
    }
    #[doc = "0x1314 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h201"]
    #[inline(always)]
    pub const fn intpriority_197(&self) -> &Intpriority197 {
        &self.intpriority_197
    }
    #[doc = "0x1318 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h202"]
    #[inline(always)]
    pub const fn intpriority_198(&self) -> &Intpriority198 {
        &self.intpriority_198
    }
    #[doc = "0x131c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h203"]
    #[inline(always)]
    pub const fn intpriority_199(&self) -> &Intpriority199 {
        &self.intpriority_199
    }
    #[doc = "0x1320 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h204"]
    #[inline(always)]
    pub const fn intpriority_200(&self) -> &Intpriority200 {
        &self.intpriority_200
    }
    #[doc = "0x1324 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h205"]
    #[inline(always)]
    pub const fn intpriority_201(&self) -> &Intpriority201 {
        &self.intpriority_201
    }
    #[doc = "0x1328 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h206"]
    #[inline(always)]
    pub const fn intpriority_202(&self) -> &Intpriority202 {
        &self.intpriority_202
    }
    #[doc = "0x132c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h207"]
    #[inline(always)]
    pub const fn intpriority_203(&self) -> &Intpriority203 {
        &self.intpriority_203
    }
    #[doc = "0x1330 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h208"]
    #[inline(always)]
    pub const fn intpriority_204(&self) -> &Intpriority204 {
        &self.intpriority_204
    }
    #[doc = "0x1334 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h209"]
    #[inline(always)]
    pub const fn intpriority_205(&self) -> &Intpriority205 {
        &self.intpriority_205
    }
    #[doc = "0x1338 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h210"]
    #[inline(always)]
    pub const fn intpriority_206(&self) -> &Intpriority206 {
        &self.intpriority_206
    }
    #[doc = "0x133c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h211"]
    #[inline(always)]
    pub const fn intpriority_207(&self) -> &Intpriority207 {
        &self.intpriority_207
    }
    #[doc = "0x1340 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h212"]
    #[inline(always)]
    pub const fn intpriority_208(&self) -> &Intpriority208 {
        &self.intpriority_208
    }
    #[doc = "0x1344 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h213"]
    #[inline(always)]
    pub const fn intpriority_209(&self) -> &Intpriority209 {
        &self.intpriority_209
    }
    #[doc = "0x1348 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h214"]
    #[inline(always)]
    pub const fn intpriority_210(&self) -> &Intpriority210 {
        &self.intpriority_210
    }
    #[doc = "0x134c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h215"]
    #[inline(always)]
    pub const fn intpriority_211(&self) -> &Intpriority211 {
        &self.intpriority_211
    }
    #[doc = "0x1350 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h216"]
    #[inline(always)]
    pub const fn intpriority_212(&self) -> &Intpriority212 {
        &self.intpriority_212
    }
    #[doc = "0x1354 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h217"]
    #[inline(always)]
    pub const fn intpriority_213(&self) -> &Intpriority213 {
        &self.intpriority_213
    }
    #[doc = "0x1358 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h218"]
    #[inline(always)]
    pub const fn intpriority_214(&self) -> &Intpriority214 {
        &self.intpriority_214
    }
    #[doc = "0x135c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h219"]
    #[inline(always)]
    pub const fn intpriority_215(&self) -> &Intpriority215 {
        &self.intpriority_215
    }
    #[doc = "0x1360 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h220"]
    #[inline(always)]
    pub const fn intpriority_216(&self) -> &Intpriority216 {
        &self.intpriority_216
    }
    #[doc = "0x1364 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h221"]
    #[inline(always)]
    pub const fn intpriority_217(&self) -> &Intpriority217 {
        &self.intpriority_217
    }
    #[doc = "0x1368 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h222"]
    #[inline(always)]
    pub const fn intpriority_218(&self) -> &Intpriority218 {
        &self.intpriority_218
    }
    #[doc = "0x136c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h223"]
    #[inline(always)]
    pub const fn intpriority_219(&self) -> &Intpriority219 {
        &self.intpriority_219
    }
    #[doc = "0x1370 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h224"]
    #[inline(always)]
    pub const fn intpriority_220(&self) -> &Intpriority220 {
        &self.intpriority_220
    }
    #[doc = "0x1374 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h225"]
    #[inline(always)]
    pub const fn intpriority_221(&self) -> &Intpriority221 {
        &self.intpriority_221
    }
    #[doc = "0x1378 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h226"]
    #[inline(always)]
    pub const fn intpriority_222(&self) -> &Intpriority222 {
        &self.intpriority_222
    }
    #[doc = "0x137c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h227"]
    #[inline(always)]
    pub const fn intpriority_223(&self) -> &Intpriority223 {
        &self.intpriority_223
    }
    #[doc = "0x1380 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h228"]
    #[inline(always)]
    pub const fn intpriority_224(&self) -> &Intpriority224 {
        &self.intpriority_224
    }
    #[doc = "0x1384 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h229"]
    #[inline(always)]
    pub const fn intpriority_225(&self) -> &Intpriority225 {
        &self.intpriority_225
    }
    #[doc = "0x1388 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h230"]
    #[inline(always)]
    pub const fn intpriority_226(&self) -> &Intpriority226 {
        &self.intpriority_226
    }
    #[doc = "0x138c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h231"]
    #[inline(always)]
    pub const fn intpriority_227(&self) -> &Intpriority227 {
        &self.intpriority_227
    }
    #[doc = "0x1390 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h232"]
    #[inline(always)]
    pub const fn intpriority_228(&self) -> &Intpriority228 {
        &self.intpriority_228
    }
    #[doc = "0x1394 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h233"]
    #[inline(always)]
    pub const fn intpriority_229(&self) -> &Intpriority229 {
        &self.intpriority_229
    }
    #[doc = "0x1398 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h234"]
    #[inline(always)]
    pub const fn intpriority_230(&self) -> &Intpriority230 {
        &self.intpriority_230
    }
    #[doc = "0x139c - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h235"]
    #[inline(always)]
    pub const fn intpriority_231(&self) -> &Intpriority231 {
        &self.intpriority_231
    }
    #[doc = "0x13a0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h236"]
    #[inline(always)]
    pub const fn intpriority_232(&self) -> &Intpriority232 {
        &self.intpriority_232
    }
    #[doc = "0x13a4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h237"]
    #[inline(always)]
    pub const fn intpriority_233(&self) -> &Intpriority233 {
        &self.intpriority_233
    }
    #[doc = "0x13a8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h238"]
    #[inline(always)]
    pub const fn intpriority_234(&self) -> &Intpriority234 {
        &self.intpriority_234
    }
    #[doc = "0x13ac - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h239"]
    #[inline(always)]
    pub const fn intpriority_235(&self) -> &Intpriority235 {
        &self.intpriority_235
    }
    #[doc = "0x13b0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h240"]
    #[inline(always)]
    pub const fn intpriority_236(&self) -> &Intpriority236 {
        &self.intpriority_236
    }
    #[doc = "0x13b4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h241"]
    #[inline(always)]
    pub const fn intpriority_237(&self) -> &Intpriority237 {
        &self.intpriority_237
    }
    #[doc = "0x13b8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h242"]
    #[inline(always)]
    pub const fn intpriority_238(&self) -> &Intpriority238 {
        &self.intpriority_238
    }
    #[doc = "0x13bc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h243"]
    #[inline(always)]
    pub const fn intpriority_239(&self) -> &Intpriority239 {
        &self.intpriority_239
    }
    #[doc = "0x13c0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h244"]
    #[inline(always)]
    pub const fn intpriority_240(&self) -> &Intpriority240 {
        &self.intpriority_240
    }
    #[doc = "0x13c4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h245"]
    #[inline(always)]
    pub const fn intpriority_241(&self) -> &Intpriority241 {
        &self.intpriority_241
    }
    #[doc = "0x13c8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h246"]
    #[inline(always)]
    pub const fn intpriority_242(&self) -> &Intpriority242 {
        &self.intpriority_242
    }
    #[doc = "0x13cc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h247"]
    #[inline(always)]
    pub const fn intpriority_243(&self) -> &Intpriority243 {
        &self.intpriority_243
    }
    #[doc = "0x13d0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h248"]
    #[inline(always)]
    pub const fn intpriority_244(&self) -> &Intpriority244 {
        &self.intpriority_244
    }
    #[doc = "0x13d4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h249"]
    #[inline(always)]
    pub const fn intpriority_245(&self) -> &Intpriority245 {
        &self.intpriority_245
    }
    #[doc = "0x13d8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h250"]
    #[inline(always)]
    pub const fn intpriority_246(&self) -> &Intpriority246 {
        &self.intpriority_246
    }
    #[doc = "0x13dc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h251"]
    #[inline(always)]
    pub const fn intpriority_247(&self) -> &Intpriority247 {
        &self.intpriority_247
    }
    #[doc = "0x13e0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h252"]
    #[inline(always)]
    pub const fn intpriority_248(&self) -> &Intpriority248 {
        &self.intpriority_248
    }
    #[doc = "0x13e4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h253"]
    #[inline(always)]
    pub const fn intpriority_249(&self) -> &Intpriority249 {
        &self.intpriority_249
    }
    #[doc = "0x13e8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h254"]
    #[inline(always)]
    pub const fn intpriority_250(&self) -> &Intpriority250 {
        &self.intpriority_250
    }
    #[doc = "0x13ec - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h255"]
    #[inline(always)]
    pub const fn intpriority_251(&self) -> &Intpriority251 {
        &self.intpriority_251
    }
    #[doc = "0x13f0 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h256"]
    #[inline(always)]
    pub const fn intpriority_252(&self) -> &Intpriority252 {
        &self.intpriority_252
    }
    #[doc = "0x13f4 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h257"]
    #[inline(always)]
    pub const fn intpriority_253(&self) -> &Intpriority253 {
        &self.intpriority_253
    }
    #[doc = "0x13f8 - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h258"]
    #[inline(always)]
    pub const fn intpriority_254(&self) -> &Intpriority254 {
        &self.intpriority_254
    }
    #[doc = "0x13fc - Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h259"]
    #[inline(always)]
    pub const fn intpriority_255(&self) -> &Intpriority255 {
        &self.intpriority_255
    }
    #[doc = "0x2000 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h4"]
    #[inline(always)]
    pub const fn intvector(&self) -> &Intvector {
        &self.intvector
    }
    #[doc = "0x2004 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h5"]
    #[inline(always)]
    pub const fn intvector_1(&self) -> &Intvector1 {
        &self.intvector_1
    }
    #[doc = "0x2008 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h6"]
    #[inline(always)]
    pub const fn intvector_2(&self) -> &Intvector2 {
        &self.intvector_2
    }
    #[doc = "0x200c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h7"]
    #[inline(always)]
    pub const fn intvector_3(&self) -> &Intvector3 {
        &self.intvector_3
    }
    #[doc = "0x2010 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h8"]
    #[inline(always)]
    pub const fn intvector_4(&self) -> &Intvector4 {
        &self.intvector_4
    }
    #[doc = "0x2014 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h9"]
    #[inline(always)]
    pub const fn intvector_5(&self) -> &Intvector5 {
        &self.intvector_5
    }
    #[doc = "0x2018 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h10"]
    #[inline(always)]
    pub const fn intvector_6(&self) -> &Intvector6 {
        &self.intvector_6
    }
    #[doc = "0x201c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h11"]
    #[inline(always)]
    pub const fn intvector_7(&self) -> &Intvector7 {
        &self.intvector_7
    }
    #[doc = "0x2020 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h12"]
    #[inline(always)]
    pub const fn intvector_8(&self) -> &Intvector8 {
        &self.intvector_8
    }
    #[doc = "0x2024 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h13"]
    #[inline(always)]
    pub const fn intvector_9(&self) -> &Intvector9 {
        &self.intvector_9
    }
    #[doc = "0x2028 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h14"]
    #[inline(always)]
    pub const fn intvector_10(&self) -> &Intvector10 {
        &self.intvector_10
    }
    #[doc = "0x202c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h15"]
    #[inline(always)]
    pub const fn intvector_11(&self) -> &Intvector11 {
        &self.intvector_11
    }
    #[doc = "0x2030 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h16"]
    #[inline(always)]
    pub const fn intvector_12(&self) -> &Intvector12 {
        &self.intvector_12
    }
    #[doc = "0x2034 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h17"]
    #[inline(always)]
    pub const fn intvector_13(&self) -> &Intvector13 {
        &self.intvector_13
    }
    #[doc = "0x2038 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h18"]
    #[inline(always)]
    pub const fn intvector_14(&self) -> &Intvector14 {
        &self.intvector_14
    }
    #[doc = "0x203c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h19"]
    #[inline(always)]
    pub const fn intvector_15(&self) -> &Intvector15 {
        &self.intvector_15
    }
    #[doc = "0x2040 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h20"]
    #[inline(always)]
    pub const fn intvector_16(&self) -> &Intvector16 {
        &self.intvector_16
    }
    #[doc = "0x2044 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h21"]
    #[inline(always)]
    pub const fn intvector_17(&self) -> &Intvector17 {
        &self.intvector_17
    }
    #[doc = "0x2048 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h22"]
    #[inline(always)]
    pub const fn intvector_18(&self) -> &Intvector18 {
        &self.intvector_18
    }
    #[doc = "0x204c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h23"]
    #[inline(always)]
    pub const fn intvector_19(&self) -> &Intvector19 {
        &self.intvector_19
    }
    #[doc = "0x2050 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h24"]
    #[inline(always)]
    pub const fn intvector_20(&self) -> &Intvector20 {
        &self.intvector_20
    }
    #[doc = "0x2054 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h25"]
    #[inline(always)]
    pub const fn intvector_21(&self) -> &Intvector21 {
        &self.intvector_21
    }
    #[doc = "0x2058 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h26"]
    #[inline(always)]
    pub const fn intvector_22(&self) -> &Intvector22 {
        &self.intvector_22
    }
    #[doc = "0x205c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h27"]
    #[inline(always)]
    pub const fn intvector_23(&self) -> &Intvector23 {
        &self.intvector_23
    }
    #[doc = "0x2060 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h28"]
    #[inline(always)]
    pub const fn intvector_24(&self) -> &Intvector24 {
        &self.intvector_24
    }
    #[doc = "0x2064 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h29"]
    #[inline(always)]
    pub const fn intvector_25(&self) -> &Intvector25 {
        &self.intvector_25
    }
    #[doc = "0x2068 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h30"]
    #[inline(always)]
    pub const fn intvector_26(&self) -> &Intvector26 {
        &self.intvector_26
    }
    #[doc = "0x206c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h31"]
    #[inline(always)]
    pub const fn intvector_27(&self) -> &Intvector27 {
        &self.intvector_27
    }
    #[doc = "0x2070 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h32"]
    #[inline(always)]
    pub const fn intvector_28(&self) -> &Intvector28 {
        &self.intvector_28
    }
    #[doc = "0x2074 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h33"]
    #[inline(always)]
    pub const fn intvector_29(&self) -> &Intvector29 {
        &self.intvector_29
    }
    #[doc = "0x2078 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h34"]
    #[inline(always)]
    pub const fn intvector_30(&self) -> &Intvector30 {
        &self.intvector_30
    }
    #[doc = "0x207c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h35"]
    #[inline(always)]
    pub const fn intvector_31(&self) -> &Intvector31 {
        &self.intvector_31
    }
    #[doc = "0x2080 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h36"]
    #[inline(always)]
    pub const fn intvector_32(&self) -> &Intvector32 {
        &self.intvector_32
    }
    #[doc = "0x2084 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h37"]
    #[inline(always)]
    pub const fn intvector_33(&self) -> &Intvector33 {
        &self.intvector_33
    }
    #[doc = "0x2088 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h38"]
    #[inline(always)]
    pub const fn intvector_34(&self) -> &Intvector34 {
        &self.intvector_34
    }
    #[doc = "0x208c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h39"]
    #[inline(always)]
    pub const fn intvector_35(&self) -> &Intvector35 {
        &self.intvector_35
    }
    #[doc = "0x2090 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h40"]
    #[inline(always)]
    pub const fn intvector_36(&self) -> &Intvector36 {
        &self.intvector_36
    }
    #[doc = "0x2094 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h41"]
    #[inline(always)]
    pub const fn intvector_37(&self) -> &Intvector37 {
        &self.intvector_37
    }
    #[doc = "0x2098 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h42"]
    #[inline(always)]
    pub const fn intvector_38(&self) -> &Intvector38 {
        &self.intvector_38
    }
    #[doc = "0x209c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h43"]
    #[inline(always)]
    pub const fn intvector_39(&self) -> &Intvector39 {
        &self.intvector_39
    }
    #[doc = "0x20a0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h44"]
    #[inline(always)]
    pub const fn intvector_40(&self) -> &Intvector40 {
        &self.intvector_40
    }
    #[doc = "0x20a4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h45"]
    #[inline(always)]
    pub const fn intvector_41(&self) -> &Intvector41 {
        &self.intvector_41
    }
    #[doc = "0x20a8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h46"]
    #[inline(always)]
    pub const fn intvector_42(&self) -> &Intvector42 {
        &self.intvector_42
    }
    #[doc = "0x20ac - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h47"]
    #[inline(always)]
    pub const fn intvector_43(&self) -> &Intvector43 {
        &self.intvector_43
    }
    #[doc = "0x20b0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h48"]
    #[inline(always)]
    pub const fn intvector_44(&self) -> &Intvector44 {
        &self.intvector_44
    }
    #[doc = "0x20b4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h49"]
    #[inline(always)]
    pub const fn intvector_45(&self) -> &Intvector45 {
        &self.intvector_45
    }
    #[doc = "0x20b8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h50"]
    #[inline(always)]
    pub const fn intvector_46(&self) -> &Intvector46 {
        &self.intvector_46
    }
    #[doc = "0x20bc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h51"]
    #[inline(always)]
    pub const fn intvector_47(&self) -> &Intvector47 {
        &self.intvector_47
    }
    #[doc = "0x20c0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h52"]
    #[inline(always)]
    pub const fn intvector_48(&self) -> &Intvector48 {
        &self.intvector_48
    }
    #[doc = "0x20c4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h53"]
    #[inline(always)]
    pub const fn intvector_49(&self) -> &Intvector49 {
        &self.intvector_49
    }
    #[doc = "0x20c8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h54"]
    #[inline(always)]
    pub const fn intvector_50(&self) -> &Intvector50 {
        &self.intvector_50
    }
    #[doc = "0x20cc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h55"]
    #[inline(always)]
    pub const fn intvector_51(&self) -> &Intvector51 {
        &self.intvector_51
    }
    #[doc = "0x20d0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h56"]
    #[inline(always)]
    pub const fn intvector_52(&self) -> &Intvector52 {
        &self.intvector_52
    }
    #[doc = "0x20d4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h57"]
    #[inline(always)]
    pub const fn intvector_53(&self) -> &Intvector53 {
        &self.intvector_53
    }
    #[doc = "0x20d8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h58"]
    #[inline(always)]
    pub const fn intvector_54(&self) -> &Intvector54 {
        &self.intvector_54
    }
    #[doc = "0x20dc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h59"]
    #[inline(always)]
    pub const fn intvector_55(&self) -> &Intvector55 {
        &self.intvector_55
    }
    #[doc = "0x20e0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h60"]
    #[inline(always)]
    pub const fn intvector_56(&self) -> &Intvector56 {
        &self.intvector_56
    }
    #[doc = "0x20e4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h61"]
    #[inline(always)]
    pub const fn intvector_57(&self) -> &Intvector57 {
        &self.intvector_57
    }
    #[doc = "0x20e8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h62"]
    #[inline(always)]
    pub const fn intvector_58(&self) -> &Intvector58 {
        &self.intvector_58
    }
    #[doc = "0x20ec - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h63"]
    #[inline(always)]
    pub const fn intvector_59(&self) -> &Intvector59 {
        &self.intvector_59
    }
    #[doc = "0x20f0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h64"]
    #[inline(always)]
    pub const fn intvector_60(&self) -> &Intvector60 {
        &self.intvector_60
    }
    #[doc = "0x20f4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h65"]
    #[inline(always)]
    pub const fn intvector_61(&self) -> &Intvector61 {
        &self.intvector_61
    }
    #[doc = "0x20f8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h66"]
    #[inline(always)]
    pub const fn intvector_62(&self) -> &Intvector62 {
        &self.intvector_62
    }
    #[doc = "0x20fc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h67"]
    #[inline(always)]
    pub const fn intvector_63(&self) -> &Intvector63 {
        &self.intvector_63
    }
    #[doc = "0x2100 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h68"]
    #[inline(always)]
    pub const fn intvector_64(&self) -> &Intvector64 {
        &self.intvector_64
    }
    #[doc = "0x2104 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h69"]
    #[inline(always)]
    pub const fn intvector_65(&self) -> &Intvector65 {
        &self.intvector_65
    }
    #[doc = "0x2108 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h70"]
    #[inline(always)]
    pub const fn intvector_66(&self) -> &Intvector66 {
        &self.intvector_66
    }
    #[doc = "0x210c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h71"]
    #[inline(always)]
    pub const fn intvector_67(&self) -> &Intvector67 {
        &self.intvector_67
    }
    #[doc = "0x2110 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h72"]
    #[inline(always)]
    pub const fn intvector_68(&self) -> &Intvector68 {
        &self.intvector_68
    }
    #[doc = "0x2114 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h73"]
    #[inline(always)]
    pub const fn intvector_69(&self) -> &Intvector69 {
        &self.intvector_69
    }
    #[doc = "0x2118 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h74"]
    #[inline(always)]
    pub const fn intvector_70(&self) -> &Intvector70 {
        &self.intvector_70
    }
    #[doc = "0x211c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h75"]
    #[inline(always)]
    pub const fn intvector_71(&self) -> &Intvector71 {
        &self.intvector_71
    }
    #[doc = "0x2120 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h76"]
    #[inline(always)]
    pub const fn intvector_72(&self) -> &Intvector72 {
        &self.intvector_72
    }
    #[doc = "0x2124 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h77"]
    #[inline(always)]
    pub const fn intvector_73(&self) -> &Intvector73 {
        &self.intvector_73
    }
    #[doc = "0x2128 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h78"]
    #[inline(always)]
    pub const fn intvector_74(&self) -> &Intvector74 {
        &self.intvector_74
    }
    #[doc = "0x212c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h79"]
    #[inline(always)]
    pub const fn intvector_75(&self) -> &Intvector75 {
        &self.intvector_75
    }
    #[doc = "0x2130 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h80"]
    #[inline(always)]
    pub const fn intvector_76(&self) -> &Intvector76 {
        &self.intvector_76
    }
    #[doc = "0x2134 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h81"]
    #[inline(always)]
    pub const fn intvector_77(&self) -> &Intvector77 {
        &self.intvector_77
    }
    #[doc = "0x2138 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h82"]
    #[inline(always)]
    pub const fn intvector_78(&self) -> &Intvector78 {
        &self.intvector_78
    }
    #[doc = "0x213c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h83"]
    #[inline(always)]
    pub const fn intvector_79(&self) -> &Intvector79 {
        &self.intvector_79
    }
    #[doc = "0x2140 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h84"]
    #[inline(always)]
    pub const fn intvector_80(&self) -> &Intvector80 {
        &self.intvector_80
    }
    #[doc = "0x2144 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h85"]
    #[inline(always)]
    pub const fn intvector_81(&self) -> &Intvector81 {
        &self.intvector_81
    }
    #[doc = "0x2148 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h86"]
    #[inline(always)]
    pub const fn intvector_82(&self) -> &Intvector82 {
        &self.intvector_82
    }
    #[doc = "0x214c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h87"]
    #[inline(always)]
    pub const fn intvector_83(&self) -> &Intvector83 {
        &self.intvector_83
    }
    #[doc = "0x2150 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h88"]
    #[inline(always)]
    pub const fn intvector_84(&self) -> &Intvector84 {
        &self.intvector_84
    }
    #[doc = "0x2154 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h89"]
    #[inline(always)]
    pub const fn intvector_85(&self) -> &Intvector85 {
        &self.intvector_85
    }
    #[doc = "0x2158 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h90"]
    #[inline(always)]
    pub const fn intvector_86(&self) -> &Intvector86 {
        &self.intvector_86
    }
    #[doc = "0x215c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h91"]
    #[inline(always)]
    pub const fn intvector_87(&self) -> &Intvector87 {
        &self.intvector_87
    }
    #[doc = "0x2160 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h92"]
    #[inline(always)]
    pub const fn intvector_88(&self) -> &Intvector88 {
        &self.intvector_88
    }
    #[doc = "0x2164 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h93"]
    #[inline(always)]
    pub const fn intvector_89(&self) -> &Intvector89 {
        &self.intvector_89
    }
    #[doc = "0x2168 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h94"]
    #[inline(always)]
    pub const fn intvector_90(&self) -> &Intvector90 {
        &self.intvector_90
    }
    #[doc = "0x216c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h95"]
    #[inline(always)]
    pub const fn intvector_91(&self) -> &Intvector91 {
        &self.intvector_91
    }
    #[doc = "0x2170 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h96"]
    #[inline(always)]
    pub const fn intvector_92(&self) -> &Intvector92 {
        &self.intvector_92
    }
    #[doc = "0x2174 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h97"]
    #[inline(always)]
    pub const fn intvector_93(&self) -> &Intvector93 {
        &self.intvector_93
    }
    #[doc = "0x2178 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h98"]
    #[inline(always)]
    pub const fn intvector_94(&self) -> &Intvector94 {
        &self.intvector_94
    }
    #[doc = "0x217c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h99"]
    #[inline(always)]
    pub const fn intvector_95(&self) -> &Intvector95 {
        &self.intvector_95
    }
    #[doc = "0x2180 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h100"]
    #[inline(always)]
    pub const fn intvector_96(&self) -> &Intvector96 {
        &self.intvector_96
    }
    #[doc = "0x2184 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h101"]
    #[inline(always)]
    pub const fn intvector_97(&self) -> &Intvector97 {
        &self.intvector_97
    }
    #[doc = "0x2188 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h102"]
    #[inline(always)]
    pub const fn intvector_98(&self) -> &Intvector98 {
        &self.intvector_98
    }
    #[doc = "0x218c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h103"]
    #[inline(always)]
    pub const fn intvector_99(&self) -> &Intvector99 {
        &self.intvector_99
    }
    #[doc = "0x2190 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h104"]
    #[inline(always)]
    pub const fn intvector_100(&self) -> &Intvector100 {
        &self.intvector_100
    }
    #[doc = "0x2194 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h105"]
    #[inline(always)]
    pub const fn intvector_101(&self) -> &Intvector101 {
        &self.intvector_101
    }
    #[doc = "0x2198 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h106"]
    #[inline(always)]
    pub const fn intvector_102(&self) -> &Intvector102 {
        &self.intvector_102
    }
    #[doc = "0x219c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h107"]
    #[inline(always)]
    pub const fn intvector_103(&self) -> &Intvector103 {
        &self.intvector_103
    }
    #[doc = "0x21a0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h108"]
    #[inline(always)]
    pub const fn intvector_104(&self) -> &Intvector104 {
        &self.intvector_104
    }
    #[doc = "0x21a4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h109"]
    #[inline(always)]
    pub const fn intvector_105(&self) -> &Intvector105 {
        &self.intvector_105
    }
    #[doc = "0x21a8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h110"]
    #[inline(always)]
    pub const fn intvector_106(&self) -> &Intvector106 {
        &self.intvector_106
    }
    #[doc = "0x21ac - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h111"]
    #[inline(always)]
    pub const fn intvector_107(&self) -> &Intvector107 {
        &self.intvector_107
    }
    #[doc = "0x21b0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h112"]
    #[inline(always)]
    pub const fn intvector_108(&self) -> &Intvector108 {
        &self.intvector_108
    }
    #[doc = "0x21b4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h113"]
    #[inline(always)]
    pub const fn intvector_109(&self) -> &Intvector109 {
        &self.intvector_109
    }
    #[doc = "0x21b8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h114"]
    #[inline(always)]
    pub const fn intvector_110(&self) -> &Intvector110 {
        &self.intvector_110
    }
    #[doc = "0x21bc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h115"]
    #[inline(always)]
    pub const fn intvector_111(&self) -> &Intvector111 {
        &self.intvector_111
    }
    #[doc = "0x21c0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h116"]
    #[inline(always)]
    pub const fn intvector_112(&self) -> &Intvector112 {
        &self.intvector_112
    }
    #[doc = "0x21c4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h117"]
    #[inline(always)]
    pub const fn intvector_113(&self) -> &Intvector113 {
        &self.intvector_113
    }
    #[doc = "0x21c8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h118"]
    #[inline(always)]
    pub const fn intvector_114(&self) -> &Intvector114 {
        &self.intvector_114
    }
    #[doc = "0x21cc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h119"]
    #[inline(always)]
    pub const fn intvector_115(&self) -> &Intvector115 {
        &self.intvector_115
    }
    #[doc = "0x21d0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h120"]
    #[inline(always)]
    pub const fn intvector_116(&self) -> &Intvector116 {
        &self.intvector_116
    }
    #[doc = "0x21d4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h121"]
    #[inline(always)]
    pub const fn intvector_117(&self) -> &Intvector117 {
        &self.intvector_117
    }
    #[doc = "0x21d8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h122"]
    #[inline(always)]
    pub const fn intvector_118(&self) -> &Intvector118 {
        &self.intvector_118
    }
    #[doc = "0x21dc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h123"]
    #[inline(always)]
    pub const fn intvector_119(&self) -> &Intvector119 {
        &self.intvector_119
    }
    #[doc = "0x21e0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h124"]
    #[inline(always)]
    pub const fn intvector_120(&self) -> &Intvector120 {
        &self.intvector_120
    }
    #[doc = "0x21e4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h125"]
    #[inline(always)]
    pub const fn intvector_121(&self) -> &Intvector121 {
        &self.intvector_121
    }
    #[doc = "0x21e8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h126"]
    #[inline(always)]
    pub const fn intvector_122(&self) -> &Intvector122 {
        &self.intvector_122
    }
    #[doc = "0x21ec - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h127"]
    #[inline(always)]
    pub const fn intvector_123(&self) -> &Intvector123 {
        &self.intvector_123
    }
    #[doc = "0x21f0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h128"]
    #[inline(always)]
    pub const fn intvector_124(&self) -> &Intvector124 {
        &self.intvector_124
    }
    #[doc = "0x21f4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h129"]
    #[inline(always)]
    pub const fn intvector_125(&self) -> &Intvector125 {
        &self.intvector_125
    }
    #[doc = "0x21f8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h130"]
    #[inline(always)]
    pub const fn intvector_126(&self) -> &Intvector126 {
        &self.intvector_126
    }
    #[doc = "0x21fc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h131"]
    #[inline(always)]
    pub const fn intvector_127(&self) -> &Intvector127 {
        &self.intvector_127
    }
    #[doc = "0x2200 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h132"]
    #[inline(always)]
    pub const fn intvector_128(&self) -> &Intvector128 {
        &self.intvector_128
    }
    #[doc = "0x2204 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h133"]
    #[inline(always)]
    pub const fn intvector_129(&self) -> &Intvector129 {
        &self.intvector_129
    }
    #[doc = "0x2208 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h134"]
    #[inline(always)]
    pub const fn intvector_130(&self) -> &Intvector130 {
        &self.intvector_130
    }
    #[doc = "0x220c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h135"]
    #[inline(always)]
    pub const fn intvector_131(&self) -> &Intvector131 {
        &self.intvector_131
    }
    #[doc = "0x2210 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h136"]
    #[inline(always)]
    pub const fn intvector_132(&self) -> &Intvector132 {
        &self.intvector_132
    }
    #[doc = "0x2214 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h137"]
    #[inline(always)]
    pub const fn intvector_133(&self) -> &Intvector133 {
        &self.intvector_133
    }
    #[doc = "0x2218 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h138"]
    #[inline(always)]
    pub const fn intvector_134(&self) -> &Intvector134 {
        &self.intvector_134
    }
    #[doc = "0x221c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h139"]
    #[inline(always)]
    pub const fn intvector_135(&self) -> &Intvector135 {
        &self.intvector_135
    }
    #[doc = "0x2220 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h140"]
    #[inline(always)]
    pub const fn intvector_136(&self) -> &Intvector136 {
        &self.intvector_136
    }
    #[doc = "0x2224 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h141"]
    #[inline(always)]
    pub const fn intvector_137(&self) -> &Intvector137 {
        &self.intvector_137
    }
    #[doc = "0x2228 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h142"]
    #[inline(always)]
    pub const fn intvector_138(&self) -> &Intvector138 {
        &self.intvector_138
    }
    #[doc = "0x222c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h143"]
    #[inline(always)]
    pub const fn intvector_139(&self) -> &Intvector139 {
        &self.intvector_139
    }
    #[doc = "0x2230 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h144"]
    #[inline(always)]
    pub const fn intvector_140(&self) -> &Intvector140 {
        &self.intvector_140
    }
    #[doc = "0x2234 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h145"]
    #[inline(always)]
    pub const fn intvector_141(&self) -> &Intvector141 {
        &self.intvector_141
    }
    #[doc = "0x2238 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h146"]
    #[inline(always)]
    pub const fn intvector_142(&self) -> &Intvector142 {
        &self.intvector_142
    }
    #[doc = "0x223c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h147"]
    #[inline(always)]
    pub const fn intvector_143(&self) -> &Intvector143 {
        &self.intvector_143
    }
    #[doc = "0x2240 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h148"]
    #[inline(always)]
    pub const fn intvector_144(&self) -> &Intvector144 {
        &self.intvector_144
    }
    #[doc = "0x2244 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h149"]
    #[inline(always)]
    pub const fn intvector_145(&self) -> &Intvector145 {
        &self.intvector_145
    }
    #[doc = "0x2248 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h150"]
    #[inline(always)]
    pub const fn intvector_146(&self) -> &Intvector146 {
        &self.intvector_146
    }
    #[doc = "0x224c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h151"]
    #[inline(always)]
    pub const fn intvector_147(&self) -> &Intvector147 {
        &self.intvector_147
    }
    #[doc = "0x2250 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h152"]
    #[inline(always)]
    pub const fn intvector_148(&self) -> &Intvector148 {
        &self.intvector_148
    }
    #[doc = "0x2254 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h153"]
    #[inline(always)]
    pub const fn intvector_149(&self) -> &Intvector149 {
        &self.intvector_149
    }
    #[doc = "0x2258 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h154"]
    #[inline(always)]
    pub const fn intvector_150(&self) -> &Intvector150 {
        &self.intvector_150
    }
    #[doc = "0x225c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h155"]
    #[inline(always)]
    pub const fn intvector_151(&self) -> &Intvector151 {
        &self.intvector_151
    }
    #[doc = "0x2260 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h156"]
    #[inline(always)]
    pub const fn intvector_152(&self) -> &Intvector152 {
        &self.intvector_152
    }
    #[doc = "0x2264 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h157"]
    #[inline(always)]
    pub const fn intvector_153(&self) -> &Intvector153 {
        &self.intvector_153
    }
    #[doc = "0x2268 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h158"]
    #[inline(always)]
    pub const fn intvector_154(&self) -> &Intvector154 {
        &self.intvector_154
    }
    #[doc = "0x226c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h159"]
    #[inline(always)]
    pub const fn intvector_155(&self) -> &Intvector155 {
        &self.intvector_155
    }
    #[doc = "0x2270 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h160"]
    #[inline(always)]
    pub const fn intvector_156(&self) -> &Intvector156 {
        &self.intvector_156
    }
    #[doc = "0x2274 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h161"]
    #[inline(always)]
    pub const fn intvector_157(&self) -> &Intvector157 {
        &self.intvector_157
    }
    #[doc = "0x2278 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h162"]
    #[inline(always)]
    pub const fn intvector_158(&self) -> &Intvector158 {
        &self.intvector_158
    }
    #[doc = "0x227c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h163"]
    #[inline(always)]
    pub const fn intvector_159(&self) -> &Intvector159 {
        &self.intvector_159
    }
    #[doc = "0x2280 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h164"]
    #[inline(always)]
    pub const fn intvector_160(&self) -> &Intvector160 {
        &self.intvector_160
    }
    #[doc = "0x2284 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h165"]
    #[inline(always)]
    pub const fn intvector_161(&self) -> &Intvector161 {
        &self.intvector_161
    }
    #[doc = "0x2288 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h166"]
    #[inline(always)]
    pub const fn intvector_162(&self) -> &Intvector162 {
        &self.intvector_162
    }
    #[doc = "0x228c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h167"]
    #[inline(always)]
    pub const fn intvector_163(&self) -> &Intvector163 {
        &self.intvector_163
    }
    #[doc = "0x2290 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h168"]
    #[inline(always)]
    pub const fn intvector_164(&self) -> &Intvector164 {
        &self.intvector_164
    }
    #[doc = "0x2294 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h169"]
    #[inline(always)]
    pub const fn intvector_165(&self) -> &Intvector165 {
        &self.intvector_165
    }
    #[doc = "0x2298 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h170"]
    #[inline(always)]
    pub const fn intvector_166(&self) -> &Intvector166 {
        &self.intvector_166
    }
    #[doc = "0x229c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h171"]
    #[inline(always)]
    pub const fn intvector_167(&self) -> &Intvector167 {
        &self.intvector_167
    }
    #[doc = "0x22a0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h172"]
    #[inline(always)]
    pub const fn intvector_168(&self) -> &Intvector168 {
        &self.intvector_168
    }
    #[doc = "0x22a4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h173"]
    #[inline(always)]
    pub const fn intvector_169(&self) -> &Intvector169 {
        &self.intvector_169
    }
    #[doc = "0x22a8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h174"]
    #[inline(always)]
    pub const fn intvector_170(&self) -> &Intvector170 {
        &self.intvector_170
    }
    #[doc = "0x22ac - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h175"]
    #[inline(always)]
    pub const fn intvector_171(&self) -> &Intvector171 {
        &self.intvector_171
    }
    #[doc = "0x22b0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h176"]
    #[inline(always)]
    pub const fn intvector_172(&self) -> &Intvector172 {
        &self.intvector_172
    }
    #[doc = "0x22b4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h177"]
    #[inline(always)]
    pub const fn intvector_173(&self) -> &Intvector173 {
        &self.intvector_173
    }
    #[doc = "0x22b8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h178"]
    #[inline(always)]
    pub const fn intvector_174(&self) -> &Intvector174 {
        &self.intvector_174
    }
    #[doc = "0x22bc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h179"]
    #[inline(always)]
    pub const fn intvector_175(&self) -> &Intvector175 {
        &self.intvector_175
    }
    #[doc = "0x22c0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h180"]
    #[inline(always)]
    pub const fn intvector_176(&self) -> &Intvector176 {
        &self.intvector_176
    }
    #[doc = "0x22c4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h181"]
    #[inline(always)]
    pub const fn intvector_177(&self) -> &Intvector177 {
        &self.intvector_177
    }
    #[doc = "0x22c8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h182"]
    #[inline(always)]
    pub const fn intvector_178(&self) -> &Intvector178 {
        &self.intvector_178
    }
    #[doc = "0x22cc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h183"]
    #[inline(always)]
    pub const fn intvector_179(&self) -> &Intvector179 {
        &self.intvector_179
    }
    #[doc = "0x22d0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h184"]
    #[inline(always)]
    pub const fn intvector_180(&self) -> &Intvector180 {
        &self.intvector_180
    }
    #[doc = "0x22d4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h185"]
    #[inline(always)]
    pub const fn intvector_181(&self) -> &Intvector181 {
        &self.intvector_181
    }
    #[doc = "0x22d8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h186"]
    #[inline(always)]
    pub const fn intvector_182(&self) -> &Intvector182 {
        &self.intvector_182
    }
    #[doc = "0x22dc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h187"]
    #[inline(always)]
    pub const fn intvector_183(&self) -> &Intvector183 {
        &self.intvector_183
    }
    #[doc = "0x22e0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h188"]
    #[inline(always)]
    pub const fn intvector_184(&self) -> &Intvector184 {
        &self.intvector_184
    }
    #[doc = "0x22e4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h189"]
    #[inline(always)]
    pub const fn intvector_185(&self) -> &Intvector185 {
        &self.intvector_185
    }
    #[doc = "0x22e8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h190"]
    #[inline(always)]
    pub const fn intvector_186(&self) -> &Intvector186 {
        &self.intvector_186
    }
    #[doc = "0x22ec - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h191"]
    #[inline(always)]
    pub const fn intvector_187(&self) -> &Intvector187 {
        &self.intvector_187
    }
    #[doc = "0x22f0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h192"]
    #[inline(always)]
    pub const fn intvector_188(&self) -> &Intvector188 {
        &self.intvector_188
    }
    #[doc = "0x22f4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h193"]
    #[inline(always)]
    pub const fn intvector_189(&self) -> &Intvector189 {
        &self.intvector_189
    }
    #[doc = "0x22f8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h194"]
    #[inline(always)]
    pub const fn intvector_190(&self) -> &Intvector190 {
        &self.intvector_190
    }
    #[doc = "0x22fc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h195"]
    #[inline(always)]
    pub const fn intvector_191(&self) -> &Intvector191 {
        &self.intvector_191
    }
    #[doc = "0x2300 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h196"]
    #[inline(always)]
    pub const fn intvector_192(&self) -> &Intvector192 {
        &self.intvector_192
    }
    #[doc = "0x2304 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h197"]
    #[inline(always)]
    pub const fn intvector_193(&self) -> &Intvector193 {
        &self.intvector_193
    }
    #[doc = "0x2308 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h198"]
    #[inline(always)]
    pub const fn intvector_194(&self) -> &Intvector194 {
        &self.intvector_194
    }
    #[doc = "0x230c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h199"]
    #[inline(always)]
    pub const fn intvector_195(&self) -> &Intvector195 {
        &self.intvector_195
    }
    #[doc = "0x2310 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h200"]
    #[inline(always)]
    pub const fn intvector_196(&self) -> &Intvector196 {
        &self.intvector_196
    }
    #[doc = "0x2314 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h201"]
    #[inline(always)]
    pub const fn intvector_197(&self) -> &Intvector197 {
        &self.intvector_197
    }
    #[doc = "0x2318 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h202"]
    #[inline(always)]
    pub const fn intvector_198(&self) -> &Intvector198 {
        &self.intvector_198
    }
    #[doc = "0x231c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h203"]
    #[inline(always)]
    pub const fn intvector_199(&self) -> &Intvector199 {
        &self.intvector_199
    }
    #[doc = "0x2320 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h204"]
    #[inline(always)]
    pub const fn intvector_200(&self) -> &Intvector200 {
        &self.intvector_200
    }
    #[doc = "0x2324 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h205"]
    #[inline(always)]
    pub const fn intvector_201(&self) -> &Intvector201 {
        &self.intvector_201
    }
    #[doc = "0x2328 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h206"]
    #[inline(always)]
    pub const fn intvector_202(&self) -> &Intvector202 {
        &self.intvector_202
    }
    #[doc = "0x232c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h207"]
    #[inline(always)]
    pub const fn intvector_203(&self) -> &Intvector203 {
        &self.intvector_203
    }
    #[doc = "0x2330 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h208"]
    #[inline(always)]
    pub const fn intvector_204(&self) -> &Intvector204 {
        &self.intvector_204
    }
    #[doc = "0x2334 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h209"]
    #[inline(always)]
    pub const fn intvector_205(&self) -> &Intvector205 {
        &self.intvector_205
    }
    #[doc = "0x2338 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h210"]
    #[inline(always)]
    pub const fn intvector_206(&self) -> &Intvector206 {
        &self.intvector_206
    }
    #[doc = "0x233c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h211"]
    #[inline(always)]
    pub const fn intvector_207(&self) -> &Intvector207 {
        &self.intvector_207
    }
    #[doc = "0x2340 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h212"]
    #[inline(always)]
    pub const fn intvector_208(&self) -> &Intvector208 {
        &self.intvector_208
    }
    #[doc = "0x2344 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h213"]
    #[inline(always)]
    pub const fn intvector_209(&self) -> &Intvector209 {
        &self.intvector_209
    }
    #[doc = "0x2348 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h214"]
    #[inline(always)]
    pub const fn intvector_210(&self) -> &Intvector210 {
        &self.intvector_210
    }
    #[doc = "0x234c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h215"]
    #[inline(always)]
    pub const fn intvector_211(&self) -> &Intvector211 {
        &self.intvector_211
    }
    #[doc = "0x2350 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h216"]
    #[inline(always)]
    pub const fn intvector_212(&self) -> &Intvector212 {
        &self.intvector_212
    }
    #[doc = "0x2354 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h217"]
    #[inline(always)]
    pub const fn intvector_213(&self) -> &Intvector213 {
        &self.intvector_213
    }
    #[doc = "0x2358 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h218"]
    #[inline(always)]
    pub const fn intvector_214(&self) -> &Intvector214 {
        &self.intvector_214
    }
    #[doc = "0x235c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h219"]
    #[inline(always)]
    pub const fn intvector_215(&self) -> &Intvector215 {
        &self.intvector_215
    }
    #[doc = "0x2360 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h220"]
    #[inline(always)]
    pub const fn intvector_216(&self) -> &Intvector216 {
        &self.intvector_216
    }
    #[doc = "0x2364 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h221"]
    #[inline(always)]
    pub const fn intvector_217(&self) -> &Intvector217 {
        &self.intvector_217
    }
    #[doc = "0x2368 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h222"]
    #[inline(always)]
    pub const fn intvector_218(&self) -> &Intvector218 {
        &self.intvector_218
    }
    #[doc = "0x236c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h223"]
    #[inline(always)]
    pub const fn intvector_219(&self) -> &Intvector219 {
        &self.intvector_219
    }
    #[doc = "0x2370 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h224"]
    #[inline(always)]
    pub const fn intvector_220(&self) -> &Intvector220 {
        &self.intvector_220
    }
    #[doc = "0x2374 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h225"]
    #[inline(always)]
    pub const fn intvector_221(&self) -> &Intvector221 {
        &self.intvector_221
    }
    #[doc = "0x2378 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h226"]
    #[inline(always)]
    pub const fn intvector_222(&self) -> &Intvector222 {
        &self.intvector_222
    }
    #[doc = "0x237c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h227"]
    #[inline(always)]
    pub const fn intvector_223(&self) -> &Intvector223 {
        &self.intvector_223
    }
    #[doc = "0x2380 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h228"]
    #[inline(always)]
    pub const fn intvector_224(&self) -> &Intvector224 {
        &self.intvector_224
    }
    #[doc = "0x2384 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h229"]
    #[inline(always)]
    pub const fn intvector_225(&self) -> &Intvector225 {
        &self.intvector_225
    }
    #[doc = "0x2388 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h230"]
    #[inline(always)]
    pub const fn intvector_226(&self) -> &Intvector226 {
        &self.intvector_226
    }
    #[doc = "0x238c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h231"]
    #[inline(always)]
    pub const fn intvector_227(&self) -> &Intvector227 {
        &self.intvector_227
    }
    #[doc = "0x2390 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h232"]
    #[inline(always)]
    pub const fn intvector_228(&self) -> &Intvector228 {
        &self.intvector_228
    }
    #[doc = "0x2394 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h233"]
    #[inline(always)]
    pub const fn intvector_229(&self) -> &Intvector229 {
        &self.intvector_229
    }
    #[doc = "0x2398 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h234"]
    #[inline(always)]
    pub const fn intvector_230(&self) -> &Intvector230 {
        &self.intvector_230
    }
    #[doc = "0x239c - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h235"]
    #[inline(always)]
    pub const fn intvector_231(&self) -> &Intvector231 {
        &self.intvector_231
    }
    #[doc = "0x23a0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h236"]
    #[inline(always)]
    pub const fn intvector_232(&self) -> &Intvector232 {
        &self.intvector_232
    }
    #[doc = "0x23a4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h237"]
    #[inline(always)]
    pub const fn intvector_233(&self) -> &Intvector233 {
        &self.intvector_233
    }
    #[doc = "0x23a8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h238"]
    #[inline(always)]
    pub const fn intvector_234(&self) -> &Intvector234 {
        &self.intvector_234
    }
    #[doc = "0x23ac - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h239"]
    #[inline(always)]
    pub const fn intvector_235(&self) -> &Intvector235 {
        &self.intvector_235
    }
    #[doc = "0x23b0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h240"]
    #[inline(always)]
    pub const fn intvector_236(&self) -> &Intvector236 {
        &self.intvector_236
    }
    #[doc = "0x23b4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h241"]
    #[inline(always)]
    pub const fn intvector_237(&self) -> &Intvector237 {
        &self.intvector_237
    }
    #[doc = "0x23b8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h242"]
    #[inline(always)]
    pub const fn intvector_238(&self) -> &Intvector238 {
        &self.intvector_238
    }
    #[doc = "0x23bc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h243"]
    #[inline(always)]
    pub const fn intvector_239(&self) -> &Intvector239 {
        &self.intvector_239
    }
    #[doc = "0x23c0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h244"]
    #[inline(always)]
    pub const fn intvector_240(&self) -> &Intvector240 {
        &self.intvector_240
    }
    #[doc = "0x23c4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h245"]
    #[inline(always)]
    pub const fn intvector_241(&self) -> &Intvector241 {
        &self.intvector_241
    }
    #[doc = "0x23c8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h246"]
    #[inline(always)]
    pub const fn intvector_242(&self) -> &Intvector242 {
        &self.intvector_242
    }
    #[doc = "0x23cc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h247"]
    #[inline(always)]
    pub const fn intvector_243(&self) -> &Intvector243 {
        &self.intvector_243
    }
    #[doc = "0x23d0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h248"]
    #[inline(always)]
    pub const fn intvector_244(&self) -> &Intvector244 {
        &self.intvector_244
    }
    #[doc = "0x23d4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h249"]
    #[inline(always)]
    pub const fn intvector_245(&self) -> &Intvector245 {
        &self.intvector_245
    }
    #[doc = "0x23d8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h250"]
    #[inline(always)]
    pub const fn intvector_246(&self) -> &Intvector246 {
        &self.intvector_246
    }
    #[doc = "0x23dc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h251"]
    #[inline(always)]
    pub const fn intvector_247(&self) -> &Intvector247 {
        &self.intvector_247
    }
    #[doc = "0x23e0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h252"]
    #[inline(always)]
    pub const fn intvector_248(&self) -> &Intvector248 {
        &self.intvector_248
    }
    #[doc = "0x23e4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h253"]
    #[inline(always)]
    pub const fn intvector_249(&self) -> &Intvector249 {
        &self.intvector_249
    }
    #[doc = "0x23e8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h254"]
    #[inline(always)]
    pub const fn intvector_250(&self) -> &Intvector250 {
        &self.intvector_250
    }
    #[doc = "0x23ec - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h255"]
    #[inline(always)]
    pub const fn intvector_251(&self) -> &Intvector251 {
        &self.intvector_251
    }
    #[doc = "0x23f0 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h256"]
    #[inline(always)]
    pub const fn intvector_252(&self) -> &Intvector252 {
        &self.intvector_252
    }
    #[doc = "0x23f4 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h257"]
    #[inline(always)]
    pub const fn intvector_253(&self) -> &Intvector253 {
        &self.intvector_253
    }
    #[doc = "0x23f8 - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h258"]
    #[inline(always)]
    pub const fn intvector_254(&self) -> &Intvector254 {
        &self.intvector_254
    }
    #[doc = "0x23fc - Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h259"]
    #[inline(always)]
    pub const fn intvector_255(&self) -> &Intvector255 {
        &self.intvector_255
    }
}
#[doc = "PID (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "PID")]
pub type Pid = crate::Reg<pid::PidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod pid;
#[doc = "INFO (rw) register accessor: The Info Register gives the configuration Inforrmation of this VIM.\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info`]
module"]
#[doc(alias = "INFO")]
pub type Info = crate::Reg<info::InfoSpec>;
#[doc = "The Info Register gives the configuration Inforrmation of this VIM."]
pub mod info;
#[doc = "PRIIRQ (rw) register accessor: The Prioritized IRQ Register shows the number of the highest priority pending IRQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`priirq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priirq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priirq`]
module"]
#[doc(alias = "PRIIRQ")]
pub type Priirq = crate::Reg<priirq::PriirqSpec>;
#[doc = "The Prioritized IRQ Register shows the number of the highest priority pending IRQ."]
pub mod priirq;
#[doc = "PRIFIQ (rw) register accessor: The Prioritized FIQ Register shows the number of the highest priority pending FIQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`prifiq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prifiq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prifiq`]
module"]
#[doc(alias = "PRIFIQ")]
pub type Prifiq = crate::Reg<prifiq::PrifiqSpec>;
#[doc = "The Prioritized FIQ Register shows the number of the highest priority pending FIQ."]
pub mod prifiq;
#[doc = "IRQGSTS (rw) register accessor: The IRQ Group Status Register indicates which groups have pending IRQ interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`irqgsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqgsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqgsts`]
module"]
#[doc(alias = "IRQGSTS")]
pub type Irqgsts = crate::Reg<irqgsts::IrqgstsSpec>;
#[doc = "The IRQ Group Status Register indicates which groups have pending IRQ interrupts."]
pub mod irqgsts;
#[doc = "FIQGSTS (rw) register accessor: The FIQ Group Status Register indicates which groups have pending FIQ interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqgsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqgsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiqgsts`]
module"]
#[doc(alias = "FIQGSTS")]
pub type Fiqgsts = crate::Reg<fiqgsts::FiqgstsSpec>;
#[doc = "The FIQ Group Status Register indicates which groups have pending FIQ interrupts."]
pub mod fiqgsts;
#[doc = "IRQVEC (rw) register accessor: The IRQ Vector Address Register contains the 32-bit address of the interrupt vector for the current pendind IRQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`irqvec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqvec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqvec`]
module"]
#[doc(alias = "IRQVEC")]
pub type Irqvec = crate::Reg<irqvec::IrqvecSpec>;
#[doc = "The IRQ Vector Address Register contains the 32-bit address of the interrupt vector for the current pendind IRQ."]
pub mod irqvec;
#[doc = "FIQVEC (rw) register accessor: The FIQ Vector Address Register contains the 32-bit address of the interrupt vector for the current pendind FIQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqvec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqvec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiqvec`]
module"]
#[doc(alias = "FIQVEC")]
pub type Fiqvec = crate::Reg<fiqvec::FiqvecSpec>;
#[doc = "The FIQ Vector Address Register contains the 32-bit address of the interrupt vector for the current pendind FIQ."]
pub mod fiqvec;
#[doc = "ACTIRQ (rw) register accessor: The Active IRQ Register shows the number of the currently active IRQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`actirq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actirq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actirq`]
module"]
#[doc(alias = "ACTIRQ")]
pub type Actirq = crate::Reg<actirq::ActirqSpec>;
#[doc = "The Active IRQ Register shows the number of the currently active IRQ."]
pub mod actirq;
#[doc = "ACTFIQ (rw) register accessor: The Active FIQ Register shows the number of the currently active FIQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`actfiq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actfiq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actfiq`]
module"]
#[doc(alias = "ACTFIQ")]
pub type Actfiq = crate::Reg<actfiq::ActfiqSpec>;
#[doc = "The Active FIQ Register shows the number of the currently active FIQ."]
pub mod actfiq;
#[doc = "DEDVEC (rw) register accessor: The DED Vector Address contains a default vector address for when an uncorrectable error is detected for an active IRQ or FIQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`dedvec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dedvec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dedvec`]
module"]
#[doc(alias = "DEDVEC")]
pub type Dedvec = crate::Reg<dedvec::DedvecSpec>;
#[doc = "The DED Vector Address contains a default vector address for when an uncorrectable error is detected for an active IRQ or FIQ."]
pub mod dedvec;
#[doc = "RAW (rw) register accessor: Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00\n\nYou can [`read`](crate::Reg::read) this register and get [`raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw`]
module"]
#[doc(alias = "RAW")]
pub type Raw = crate::Reg<raw::RawSpec>;
#[doc = "Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
pub mod raw;
#[doc = "STS (rw) register accessor: Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
pub mod sts;
#[doc = "INTR_EN_SET (rw) register accessor: Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en_set`]
module"]
#[doc(alias = "INTR_EN_SET")]
pub type IntrEnSet = crate::Reg<intr_en_set::IntrEnSetSpec>;
#[doc = "Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
pub mod intr_en_set;
#[doc = "INTER_EN_CLR (rw) register accessor: Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_en_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_en_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_en_clr`]
module"]
#[doc(alias = "INTER_EN_CLR")]
pub type InterEnClr = crate::Reg<inter_en_clr::InterEnClrSpec>;
#[doc = "Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
pub mod inter_en_clr;
#[doc = "IRQSTS (rw) register accessor: Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10\n\nYou can [`read`](crate::Reg::read) this register and get [`irqsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqsts`]
module"]
#[doc(alias = "IRQSTS")]
pub type Irqsts = crate::Reg<irqsts::IrqstsSpec>;
#[doc = "Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
pub mod irqsts;
#[doc = "FIQSTS (rw) register accessor: Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiqsts`]
module"]
#[doc(alias = "FIQSTS")]
pub type Fiqsts = crate::Reg<fiqsts::FiqstsSpec>;
#[doc = "Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
pub mod fiqsts;
#[doc = "INTMAP (rw) register accessor: Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmap`]
module"]
#[doc(alias = "INTMAP")]
pub type Intmap = crate::Reg<intmap::IntmapSpec>;
#[doc = "Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
pub mod intmap;
#[doc = "INTTYPE (rw) register accessor: Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype`]
module"]
#[doc(alias = "INTTYPE")]
pub type Inttype = crate::Reg<inttype::InttypeSpec>;
#[doc = "Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
pub mod inttype;
#[doc = "RAW_1 (rw) register accessor: Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_1`]
module"]
#[doc(alias = "RAW_1")]
pub type Raw1 = crate::Reg<raw_1::Raw1Spec>;
#[doc = "Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
pub mod raw_1;
#[doc = "STS_1 (rw) register accessor: Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts_1`]
module"]
#[doc(alias = "STS_1")]
pub type Sts1 = crate::Reg<sts_1::Sts1Spec>;
#[doc = "Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
pub mod sts_1;
#[doc = "INTR_EN_SET_1 (rw) register accessor: Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_set_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_set_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en_set_1`]
module"]
#[doc(alias = "INTR_EN_SET_1")]
pub type IntrEnSet1 = crate::Reg<intr_en_set_1::IntrEnSet1Spec>;
#[doc = "Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
pub mod intr_en_set_1;
#[doc = "INTER_EN_CLR_1 (rw) register accessor: Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_en_clr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_en_clr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_en_clr_1`]
module"]
#[doc(alias = "INTER_EN_CLR_1")]
pub type InterEnClr1 = crate::Reg<inter_en_clr_1::InterEnClr1Spec>;
#[doc = "Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
pub mod inter_en_clr_1;
#[doc = "IRQSTS_1 (rw) register accessor: Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10\n\nYou can [`read`](crate::Reg::read) this register and get [`irqsts_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsts_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqsts_1`]
module"]
#[doc(alias = "IRQSTS_1")]
pub type Irqsts1 = crate::Reg<irqsts_1::Irqsts1Spec>;
#[doc = "Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
pub mod irqsts_1;
#[doc = "FIQSTS_1 (rw) register accessor: Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqsts_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqsts_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiqsts_1`]
module"]
#[doc(alias = "FIQSTS_1")]
pub type Fiqsts1 = crate::Reg<fiqsts_1::Fiqsts1Spec>;
#[doc = "Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
pub mod fiqsts_1;
#[doc = "INTMAP_1 (rw) register accessor: Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmap_1`]
module"]
#[doc(alias = "INTMAP_1")]
pub type Intmap1 = crate::Reg<intmap_1::Intmap1Spec>;
#[doc = "Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
pub mod intmap_1;
#[doc = "INTTYPE_1 (rw) register accessor: Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype_1`]
module"]
#[doc(alias = "INTTYPE_1")]
pub type Inttype1 = crate::Reg<inttype_1::Inttype1Spec>;
#[doc = "Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
pub mod inttype_1;
#[doc = "RAW_2 (rw) register accessor: Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_2`]
module"]
#[doc(alias = "RAW_2")]
pub type Raw2 = crate::Reg<raw_2::Raw2Spec>;
#[doc = "Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
pub mod raw_2;
#[doc = "STS_2 (rw) register accessor: Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts_2`]
module"]
#[doc(alias = "STS_2")]
pub type Sts2 = crate::Reg<sts_2::Sts2Spec>;
#[doc = "Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
pub mod sts_2;
#[doc = "INTR_EN_SET_2 (rw) register accessor: Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_set_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_set_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en_set_2`]
module"]
#[doc(alias = "INTR_EN_SET_2")]
pub type IntrEnSet2 = crate::Reg<intr_en_set_2::IntrEnSet2Spec>;
#[doc = "Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
pub mod intr_en_set_2;
#[doc = "INTER_EN_CLR_2 (rw) register accessor: Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_en_clr_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_en_clr_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_en_clr_2`]
module"]
#[doc(alias = "INTER_EN_CLR_2")]
pub type InterEnClr2 = crate::Reg<inter_en_clr_2::InterEnClr2Spec>;
#[doc = "Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
pub mod inter_en_clr_2;
#[doc = "IRQSTS_2 (rw) register accessor: Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10\n\nYou can [`read`](crate::Reg::read) this register and get [`irqsts_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsts_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqsts_2`]
module"]
#[doc(alias = "IRQSTS_2")]
pub type Irqsts2 = crate::Reg<irqsts_2::Irqsts2Spec>;
#[doc = "Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
pub mod irqsts_2;
#[doc = "FIQSTS_2 (rw) register accessor: Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqsts_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqsts_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiqsts_2`]
module"]
#[doc(alias = "FIQSTS_2")]
pub type Fiqsts2 = crate::Reg<fiqsts_2::Fiqsts2Spec>;
#[doc = "Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
pub mod fiqsts_2;
#[doc = "INTMAP_2 (rw) register accessor: Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmap_2`]
module"]
#[doc(alias = "INTMAP_2")]
pub type Intmap2 = crate::Reg<intmap_2::Intmap2Spec>;
#[doc = "Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
pub mod intmap_2;
#[doc = "INTTYPE_2 (rw) register accessor: Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype_2`]
module"]
#[doc(alias = "INTTYPE_2")]
pub type Inttype2 = crate::Reg<inttype_2::Inttype2Spec>;
#[doc = "Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
pub mod inttype_2;
#[doc = "RAW_3 (rw) register accessor: Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_3`]
module"]
#[doc(alias = "RAW_3")]
pub type Raw3 = crate::Reg<raw_3::Raw3Spec>;
#[doc = "Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
pub mod raw_3;
#[doc = "STS_3 (rw) register accessor: Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts_3`]
module"]
#[doc(alias = "STS_3")]
pub type Sts3 = crate::Reg<sts_3::Sts3Spec>;
#[doc = "Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
pub mod sts_3;
#[doc = "INTR_EN_SET_3 (rw) register accessor: Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_set_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_set_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en_set_3`]
module"]
#[doc(alias = "INTR_EN_SET_3")]
pub type IntrEnSet3 = crate::Reg<intr_en_set_3::IntrEnSet3Spec>;
#[doc = "Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
pub mod intr_en_set_3;
#[doc = "INTER_EN_CLR_3 (rw) register accessor: Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_en_clr_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_en_clr_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_en_clr_3`]
module"]
#[doc(alias = "INTER_EN_CLR_3")]
pub type InterEnClr3 = crate::Reg<inter_en_clr_3::InterEnClr3Spec>;
#[doc = "Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
pub mod inter_en_clr_3;
#[doc = "IRQSTS_3 (rw) register accessor: Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10\n\nYou can [`read`](crate::Reg::read) this register and get [`irqsts_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsts_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqsts_3`]
module"]
#[doc(alias = "IRQSTS_3")]
pub type Irqsts3 = crate::Reg<irqsts_3::Irqsts3Spec>;
#[doc = "Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
pub mod irqsts_3;
#[doc = "FIQSTS_3 (rw) register accessor: Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqsts_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqsts_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiqsts_3`]
module"]
#[doc(alias = "FIQSTS_3")]
pub type Fiqsts3 = crate::Reg<fiqsts_3::Fiqsts3Spec>;
#[doc = "Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
pub mod fiqsts_3;
#[doc = "INTMAP_3 (rw) register accessor: Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmap_3`]
module"]
#[doc(alias = "INTMAP_3")]
pub type Intmap3 = crate::Reg<intmap_3::Intmap3Spec>;
#[doc = "Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
pub mod intmap_3;
#[doc = "INTTYPE_3 (rw) register accessor: Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype_3`]
module"]
#[doc(alias = "INTTYPE_3")]
pub type Inttype3 = crate::Reg<inttype_3::Inttype3Spec>;
#[doc = "Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
pub mod inttype_3;
#[doc = "RAW_4 (rw) register accessor: Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_4`]
module"]
#[doc(alias = "RAW_4")]
pub type Raw4 = crate::Reg<raw_4::Raw4Spec>;
#[doc = "Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
pub mod raw_4;
#[doc = "STS_4 (rw) register accessor: Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts_4`]
module"]
#[doc(alias = "STS_4")]
pub type Sts4 = crate::Reg<sts_4::Sts4Spec>;
#[doc = "Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
pub mod sts_4;
#[doc = "INTR_EN_SET_4 (rw) register accessor: Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_set_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_set_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en_set_4`]
module"]
#[doc(alias = "INTR_EN_SET_4")]
pub type IntrEnSet4 = crate::Reg<intr_en_set_4::IntrEnSet4Spec>;
#[doc = "Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
pub mod intr_en_set_4;
#[doc = "INTER_EN_CLR_4 (rw) register accessor: Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_en_clr_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_en_clr_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_en_clr_4`]
module"]
#[doc(alias = "INTER_EN_CLR_4")]
pub type InterEnClr4 = crate::Reg<inter_en_clr_4::InterEnClr4Spec>;
#[doc = "Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
pub mod inter_en_clr_4;
#[doc = "IRQSTS_4 (rw) register accessor: Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10\n\nYou can [`read`](crate::Reg::read) this register and get [`irqsts_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsts_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqsts_4`]
module"]
#[doc(alias = "IRQSTS_4")]
pub type Irqsts4 = crate::Reg<irqsts_4::Irqsts4Spec>;
#[doc = "Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
pub mod irqsts_4;
#[doc = "FIQSTS_4 (rw) register accessor: Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqsts_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqsts_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiqsts_4`]
module"]
#[doc(alias = "FIQSTS_4")]
pub type Fiqsts4 = crate::Reg<fiqsts_4::Fiqsts4Spec>;
#[doc = "Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
pub mod fiqsts_4;
#[doc = "INTMAP_4 (rw) register accessor: Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmap_4`]
module"]
#[doc(alias = "INTMAP_4")]
pub type Intmap4 = crate::Reg<intmap_4::Intmap4Spec>;
#[doc = "Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
pub mod intmap_4;
#[doc = "INTTYPE_4 (rw) register accessor: Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype_4`]
module"]
#[doc(alias = "INTTYPE_4")]
pub type Inttype4 = crate::Reg<inttype_4::Inttype4Spec>;
#[doc = "Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
pub mod inttype_4;
#[doc = "RAW_5 (rw) register accessor: Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_5`]
module"]
#[doc(alias = "RAW_5")]
pub type Raw5 = crate::Reg<raw_5::Raw5Spec>;
#[doc = "Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
pub mod raw_5;
#[doc = "STS_5 (rw) register accessor: Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts_5`]
module"]
#[doc(alias = "STS_5")]
pub type Sts5 = crate::Reg<sts_5::Sts5Spec>;
#[doc = "Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
pub mod sts_5;
#[doc = "INTR_EN_SET_5 (rw) register accessor: Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_set_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_set_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en_set_5`]
module"]
#[doc(alias = "INTR_EN_SET_5")]
pub type IntrEnSet5 = crate::Reg<intr_en_set_5::IntrEnSet5Spec>;
#[doc = "Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
pub mod intr_en_set_5;
#[doc = "INTER_EN_CLR_5 (rw) register accessor: Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_en_clr_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_en_clr_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_en_clr_5`]
module"]
#[doc(alias = "INTER_EN_CLR_5")]
pub type InterEnClr5 = crate::Reg<inter_en_clr_5::InterEnClr5Spec>;
#[doc = "Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
pub mod inter_en_clr_5;
#[doc = "IRQSTS_5 (rw) register accessor: Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10\n\nYou can [`read`](crate::Reg::read) this register and get [`irqsts_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsts_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqsts_5`]
module"]
#[doc(alias = "IRQSTS_5")]
pub type Irqsts5 = crate::Reg<irqsts_5::Irqsts5Spec>;
#[doc = "Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
pub mod irqsts_5;
#[doc = "FIQSTS_5 (rw) register accessor: Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqsts_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqsts_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiqsts_5`]
module"]
#[doc(alias = "FIQSTS_5")]
pub type Fiqsts5 = crate::Reg<fiqsts_5::Fiqsts5Spec>;
#[doc = "Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
pub mod fiqsts_5;
#[doc = "INTMAP_5 (rw) register accessor: Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmap_5`]
module"]
#[doc(alias = "INTMAP_5")]
pub type Intmap5 = crate::Reg<intmap_5::Intmap5Spec>;
#[doc = "Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
pub mod intmap_5;
#[doc = "INTTYPE_5 (rw) register accessor: Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype_5`]
module"]
#[doc(alias = "INTTYPE_5")]
pub type Inttype5 = crate::Reg<inttype_5::Inttype5Spec>;
#[doc = "Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
pub mod inttype_5;
#[doc = "RAW_6 (rw) register accessor: Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_6`]
module"]
#[doc(alias = "RAW_6")]
pub type Raw6 = crate::Reg<raw_6::Raw6Spec>;
#[doc = "Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
pub mod raw_6;
#[doc = "STS_6 (rw) register accessor: Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts_6`]
module"]
#[doc(alias = "STS_6")]
pub type Sts6 = crate::Reg<sts_6::Sts6Spec>;
#[doc = "Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
pub mod sts_6;
#[doc = "INTR_EN_SET_6 (rw) register accessor: Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_set_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_set_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en_set_6`]
module"]
#[doc(alias = "INTR_EN_SET_6")]
pub type IntrEnSet6 = crate::Reg<intr_en_set_6::IntrEnSet6Spec>;
#[doc = "Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
pub mod intr_en_set_6;
#[doc = "INTER_EN_CLR_6 (rw) register accessor: Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_en_clr_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_en_clr_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_en_clr_6`]
module"]
#[doc(alias = "INTER_EN_CLR_6")]
pub type InterEnClr6 = crate::Reg<inter_en_clr_6::InterEnClr6Spec>;
#[doc = "Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
pub mod inter_en_clr_6;
#[doc = "IRQSTS_6 (rw) register accessor: Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10\n\nYou can [`read`](crate::Reg::read) this register and get [`irqsts_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsts_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqsts_6`]
module"]
#[doc(alias = "IRQSTS_6")]
pub type Irqsts6 = crate::Reg<irqsts_6::Irqsts6Spec>;
#[doc = "Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
pub mod irqsts_6;
#[doc = "FIQSTS_6 (rw) register accessor: Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqsts_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqsts_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiqsts_6`]
module"]
#[doc(alias = "FIQSTS_6")]
pub type Fiqsts6 = crate::Reg<fiqsts_6::Fiqsts6Spec>;
#[doc = "Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
pub mod fiqsts_6;
#[doc = "INTMAP_6 (rw) register accessor: Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmap_6`]
module"]
#[doc(alias = "INTMAP_6")]
pub type Intmap6 = crate::Reg<intmap_6::Intmap6Spec>;
#[doc = "Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
pub mod intmap_6;
#[doc = "INTTYPE_6 (rw) register accessor: Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype_6`]
module"]
#[doc(alias = "INTTYPE_6")]
pub type Inttype6 = crate::Reg<inttype_6::Inttype6Spec>;
#[doc = "Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
pub mod inttype_6;
#[doc = "RAW_7 (rw) register accessor: Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_7`]
module"]
#[doc(alias = "RAW_7")]
pub type Raw7 = crate::Reg<raw_7::Raw7Spec>;
#[doc = "Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00"]
pub mod raw_7;
#[doc = "STS_7 (rw) register accessor: Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts_7`]
module"]
#[doc(alias = "STS_7")]
pub type Sts7 = crate::Reg<sts_7::Sts7Spec>;
#[doc = "Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04"]
pub mod sts_7;
#[doc = "INTR_EN_SET_7 (rw) register accessor: Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_set_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_set_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en_set_7`]
module"]
#[doc(alias = "INTR_EN_SET_7")]
pub type IntrEnSet7 = crate::Reg<intr_en_set_7::IntrEnSet7Spec>;
#[doc = "Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08"]
pub mod intr_en_set_7;
#[doc = "INTER_EN_CLR_7 (rw) register accessor: Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_en_clr_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_en_clr_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_en_clr_7`]
module"]
#[doc(alias = "INTER_EN_CLR_7")]
pub type InterEnClr7 = crate::Reg<inter_en_clr_7::InterEnClr7Spec>;
#[doc = "Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C"]
pub mod inter_en_clr_7;
#[doc = "IRQSTS_7 (rw) register accessor: Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10\n\nYou can [`read`](crate::Reg::read) this register and get [`irqsts_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsts_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqsts_7`]
module"]
#[doc(alias = "IRQSTS_7")]
pub type Irqsts7 = crate::Reg<irqsts_7::Irqsts7Spec>;
#[doc = "Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10"]
pub mod irqsts_7;
#[doc = "FIQSTS_7 (rw) register accessor: Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqsts_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqsts_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiqsts_7`]
module"]
#[doc(alias = "FIQSTS_7")]
pub type Fiqsts7 = crate::Reg<fiqsts_7::Fiqsts7Spec>;
#[doc = "Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14"]
pub mod fiqsts_7;
#[doc = "INTMAP_7 (rw) register accessor: Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmap_7`]
module"]
#[doc(alias = "INTMAP_7")]
pub type Intmap7 = crate::Reg<intmap_7::Intmap7Spec>;
#[doc = "Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18"]
pub mod intmap_7;
#[doc = "INTTYPE_7 (rw) register accessor: Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype_7`]
module"]
#[doc(alias = "INTTYPE_7")]
pub type Inttype7 = crate::Reg<inttype_7::Inttype7Spec>;
#[doc = "Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C"]
pub mod inttype_7;
#[doc = "INTPRIORITY (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h4\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority`]
module"]
#[doc(alias = "INTPRIORITY")]
pub type Intpriority = crate::Reg<intpriority::IntprioritySpec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h4"]
pub mod intpriority;
#[doc = "INTPRIORITY_1 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h5\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_1`]
module"]
#[doc(alias = "INTPRIORITY_1")]
pub type Intpriority1 = crate::Reg<intpriority_1::Intpriority1Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h5"]
pub mod intpriority_1;
#[doc = "INTPRIORITY_2 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h6\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_2`]
module"]
#[doc(alias = "INTPRIORITY_2")]
pub type Intpriority2 = crate::Reg<intpriority_2::Intpriority2Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h6"]
pub mod intpriority_2;
#[doc = "INTPRIORITY_3 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h7\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_3`]
module"]
#[doc(alias = "INTPRIORITY_3")]
pub type Intpriority3 = crate::Reg<intpriority_3::Intpriority3Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h7"]
pub mod intpriority_3;
#[doc = "INTPRIORITY_4 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h8\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_4`]
module"]
#[doc(alias = "INTPRIORITY_4")]
pub type Intpriority4 = crate::Reg<intpriority_4::Intpriority4Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h8"]
pub mod intpriority_4;
#[doc = "INTPRIORITY_5 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h9\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_5`]
module"]
#[doc(alias = "INTPRIORITY_5")]
pub type Intpriority5 = crate::Reg<intpriority_5::Intpriority5Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h9"]
pub mod intpriority_5;
#[doc = "INTPRIORITY_6 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h10\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_6`]
module"]
#[doc(alias = "INTPRIORITY_6")]
pub type Intpriority6 = crate::Reg<intpriority_6::Intpriority6Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h10"]
pub mod intpriority_6;
#[doc = "INTPRIORITY_7 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h11\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_7`]
module"]
#[doc(alias = "INTPRIORITY_7")]
pub type Intpriority7 = crate::Reg<intpriority_7::Intpriority7Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h11"]
pub mod intpriority_7;
#[doc = "INTPRIORITY_8 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h12\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_8`]
module"]
#[doc(alias = "INTPRIORITY_8")]
pub type Intpriority8 = crate::Reg<intpriority_8::Intpriority8Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h12"]
pub mod intpriority_8;
#[doc = "INTPRIORITY_9 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h13\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_9`]
module"]
#[doc(alias = "INTPRIORITY_9")]
pub type Intpriority9 = crate::Reg<intpriority_9::Intpriority9Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h13"]
pub mod intpriority_9;
#[doc = "INTPRIORITY_10 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h14\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_10`]
module"]
#[doc(alias = "INTPRIORITY_10")]
pub type Intpriority10 = crate::Reg<intpriority_10::Intpriority10Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h14"]
pub mod intpriority_10;
#[doc = "INTPRIORITY_11 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h15\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_11`]
module"]
#[doc(alias = "INTPRIORITY_11")]
pub type Intpriority11 = crate::Reg<intpriority_11::Intpriority11Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h15"]
pub mod intpriority_11;
#[doc = "INTPRIORITY_12 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h16\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_12`]
module"]
#[doc(alias = "INTPRIORITY_12")]
pub type Intpriority12 = crate::Reg<intpriority_12::Intpriority12Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h16"]
pub mod intpriority_12;
#[doc = "INTPRIORITY_13 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h17\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_13`]
module"]
#[doc(alias = "INTPRIORITY_13")]
pub type Intpriority13 = crate::Reg<intpriority_13::Intpriority13Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h17"]
pub mod intpriority_13;
#[doc = "INTPRIORITY_14 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_14`]
module"]
#[doc(alias = "INTPRIORITY_14")]
pub type Intpriority14 = crate::Reg<intpriority_14::Intpriority14Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h18"]
pub mod intpriority_14;
#[doc = "INTPRIORITY_15 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h19\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_15`]
module"]
#[doc(alias = "INTPRIORITY_15")]
pub type Intpriority15 = crate::Reg<intpriority_15::Intpriority15Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h19"]
pub mod intpriority_15;
#[doc = "INTPRIORITY_16 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h20\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_16`]
module"]
#[doc(alias = "INTPRIORITY_16")]
pub type Intpriority16 = crate::Reg<intpriority_16::Intpriority16Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h20"]
pub mod intpriority_16;
#[doc = "INTPRIORITY_17 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h21\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_17`]
module"]
#[doc(alias = "INTPRIORITY_17")]
pub type Intpriority17 = crate::Reg<intpriority_17::Intpriority17Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h21"]
pub mod intpriority_17;
#[doc = "INTPRIORITY_18 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h22\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_18`]
module"]
#[doc(alias = "INTPRIORITY_18")]
pub type Intpriority18 = crate::Reg<intpriority_18::Intpriority18Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h22"]
pub mod intpriority_18;
#[doc = "INTPRIORITY_19 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h23\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_19`]
module"]
#[doc(alias = "INTPRIORITY_19")]
pub type Intpriority19 = crate::Reg<intpriority_19::Intpriority19Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h23"]
pub mod intpriority_19;
#[doc = "INTPRIORITY_20 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h24\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_20`]
module"]
#[doc(alias = "INTPRIORITY_20")]
pub type Intpriority20 = crate::Reg<intpriority_20::Intpriority20Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h24"]
pub mod intpriority_20;
#[doc = "INTPRIORITY_21 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h25\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_21`]
module"]
#[doc(alias = "INTPRIORITY_21")]
pub type Intpriority21 = crate::Reg<intpriority_21::Intpriority21Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h25"]
pub mod intpriority_21;
#[doc = "INTPRIORITY_22 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h26\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_22`]
module"]
#[doc(alias = "INTPRIORITY_22")]
pub type Intpriority22 = crate::Reg<intpriority_22::Intpriority22Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h26"]
pub mod intpriority_22;
#[doc = "INTPRIORITY_23 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h27\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_23`]
module"]
#[doc(alias = "INTPRIORITY_23")]
pub type Intpriority23 = crate::Reg<intpriority_23::Intpriority23Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h27"]
pub mod intpriority_23;
#[doc = "INTPRIORITY_24 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h28\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_24`]
module"]
#[doc(alias = "INTPRIORITY_24")]
pub type Intpriority24 = crate::Reg<intpriority_24::Intpriority24Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h28"]
pub mod intpriority_24;
#[doc = "INTPRIORITY_25 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h29\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_25`]
module"]
#[doc(alias = "INTPRIORITY_25")]
pub type Intpriority25 = crate::Reg<intpriority_25::Intpriority25Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h29"]
pub mod intpriority_25;
#[doc = "INTPRIORITY_26 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h30\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_26`]
module"]
#[doc(alias = "INTPRIORITY_26")]
pub type Intpriority26 = crate::Reg<intpriority_26::Intpriority26Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h30"]
pub mod intpriority_26;
#[doc = "INTPRIORITY_27 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h31\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_27`]
module"]
#[doc(alias = "INTPRIORITY_27")]
pub type Intpriority27 = crate::Reg<intpriority_27::Intpriority27Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h31"]
pub mod intpriority_27;
#[doc = "INTPRIORITY_28 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h32\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_28`]
module"]
#[doc(alias = "INTPRIORITY_28")]
pub type Intpriority28 = crate::Reg<intpriority_28::Intpriority28Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h32"]
pub mod intpriority_28;
#[doc = "INTPRIORITY_29 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h33\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_29`]
module"]
#[doc(alias = "INTPRIORITY_29")]
pub type Intpriority29 = crate::Reg<intpriority_29::Intpriority29Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h33"]
pub mod intpriority_29;
#[doc = "INTPRIORITY_30 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h34\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_30`]
module"]
#[doc(alias = "INTPRIORITY_30")]
pub type Intpriority30 = crate::Reg<intpriority_30::Intpriority30Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h34"]
pub mod intpriority_30;
#[doc = "INTPRIORITY_31 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h35\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_31`]
module"]
#[doc(alias = "INTPRIORITY_31")]
pub type Intpriority31 = crate::Reg<intpriority_31::Intpriority31Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h35"]
pub mod intpriority_31;
#[doc = "INTPRIORITY_32 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h36\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_32`]
module"]
#[doc(alias = "INTPRIORITY_32")]
pub type Intpriority32 = crate::Reg<intpriority_32::Intpriority32Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h36"]
pub mod intpriority_32;
#[doc = "INTPRIORITY_33 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h37\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_33`]
module"]
#[doc(alias = "INTPRIORITY_33")]
pub type Intpriority33 = crate::Reg<intpriority_33::Intpriority33Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h37"]
pub mod intpriority_33;
#[doc = "INTPRIORITY_34 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h38\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_34`]
module"]
#[doc(alias = "INTPRIORITY_34")]
pub type Intpriority34 = crate::Reg<intpriority_34::Intpriority34Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h38"]
pub mod intpriority_34;
#[doc = "INTPRIORITY_35 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h39\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_35`]
module"]
#[doc(alias = "INTPRIORITY_35")]
pub type Intpriority35 = crate::Reg<intpriority_35::Intpriority35Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h39"]
pub mod intpriority_35;
#[doc = "INTPRIORITY_36 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h40\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_36`]
module"]
#[doc(alias = "INTPRIORITY_36")]
pub type Intpriority36 = crate::Reg<intpriority_36::Intpriority36Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h40"]
pub mod intpriority_36;
#[doc = "INTPRIORITY_37 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h41\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_37`]
module"]
#[doc(alias = "INTPRIORITY_37")]
pub type Intpriority37 = crate::Reg<intpriority_37::Intpriority37Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h41"]
pub mod intpriority_37;
#[doc = "INTPRIORITY_38 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h42\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_38`]
module"]
#[doc(alias = "INTPRIORITY_38")]
pub type Intpriority38 = crate::Reg<intpriority_38::Intpriority38Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h42"]
pub mod intpriority_38;
#[doc = "INTPRIORITY_39 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h43\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_39`]
module"]
#[doc(alias = "INTPRIORITY_39")]
pub type Intpriority39 = crate::Reg<intpriority_39::Intpriority39Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h43"]
pub mod intpriority_39;
#[doc = "INTPRIORITY_40 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h44\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_40`]
module"]
#[doc(alias = "INTPRIORITY_40")]
pub type Intpriority40 = crate::Reg<intpriority_40::Intpriority40Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h44"]
pub mod intpriority_40;
#[doc = "INTPRIORITY_41 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h45\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_41`]
module"]
#[doc(alias = "INTPRIORITY_41")]
pub type Intpriority41 = crate::Reg<intpriority_41::Intpriority41Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h45"]
pub mod intpriority_41;
#[doc = "INTPRIORITY_42 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h46\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_42`]
module"]
#[doc(alias = "INTPRIORITY_42")]
pub type Intpriority42 = crate::Reg<intpriority_42::Intpriority42Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h46"]
pub mod intpriority_42;
#[doc = "INTPRIORITY_43 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h47\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_43`]
module"]
#[doc(alias = "INTPRIORITY_43")]
pub type Intpriority43 = crate::Reg<intpriority_43::Intpriority43Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h47"]
pub mod intpriority_43;
#[doc = "INTPRIORITY_44 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h48\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_44`]
module"]
#[doc(alias = "INTPRIORITY_44")]
pub type Intpriority44 = crate::Reg<intpriority_44::Intpriority44Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h48"]
pub mod intpriority_44;
#[doc = "INTPRIORITY_45 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h49\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_45`]
module"]
#[doc(alias = "INTPRIORITY_45")]
pub type Intpriority45 = crate::Reg<intpriority_45::Intpriority45Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h49"]
pub mod intpriority_45;
#[doc = "INTPRIORITY_46 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h50\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_46`]
module"]
#[doc(alias = "INTPRIORITY_46")]
pub type Intpriority46 = crate::Reg<intpriority_46::Intpriority46Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h50"]
pub mod intpriority_46;
#[doc = "INTPRIORITY_47 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h51\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_47`]
module"]
#[doc(alias = "INTPRIORITY_47")]
pub type Intpriority47 = crate::Reg<intpriority_47::Intpriority47Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h51"]
pub mod intpriority_47;
#[doc = "INTPRIORITY_48 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h52\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_48`]
module"]
#[doc(alias = "INTPRIORITY_48")]
pub type Intpriority48 = crate::Reg<intpriority_48::Intpriority48Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h52"]
pub mod intpriority_48;
#[doc = "INTPRIORITY_49 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h53\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_49`]
module"]
#[doc(alias = "INTPRIORITY_49")]
pub type Intpriority49 = crate::Reg<intpriority_49::Intpriority49Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h53"]
pub mod intpriority_49;
#[doc = "INTPRIORITY_50 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h54\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_50`]
module"]
#[doc(alias = "INTPRIORITY_50")]
pub type Intpriority50 = crate::Reg<intpriority_50::Intpriority50Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h54"]
pub mod intpriority_50;
#[doc = "INTPRIORITY_51 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h55\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_51`]
module"]
#[doc(alias = "INTPRIORITY_51")]
pub type Intpriority51 = crate::Reg<intpriority_51::Intpriority51Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h55"]
pub mod intpriority_51;
#[doc = "INTPRIORITY_52 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h56\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_52`]
module"]
#[doc(alias = "INTPRIORITY_52")]
pub type Intpriority52 = crate::Reg<intpriority_52::Intpriority52Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h56"]
pub mod intpriority_52;
#[doc = "INTPRIORITY_53 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h57\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_53`]
module"]
#[doc(alias = "INTPRIORITY_53")]
pub type Intpriority53 = crate::Reg<intpriority_53::Intpriority53Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h57"]
pub mod intpriority_53;
#[doc = "INTPRIORITY_54 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h58\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_54`]
module"]
#[doc(alias = "INTPRIORITY_54")]
pub type Intpriority54 = crate::Reg<intpriority_54::Intpriority54Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h58"]
pub mod intpriority_54;
#[doc = "INTPRIORITY_55 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h59\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_55`]
module"]
#[doc(alias = "INTPRIORITY_55")]
pub type Intpriority55 = crate::Reg<intpriority_55::Intpriority55Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h59"]
pub mod intpriority_55;
#[doc = "INTPRIORITY_56 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h60\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_56`]
module"]
#[doc(alias = "INTPRIORITY_56")]
pub type Intpriority56 = crate::Reg<intpriority_56::Intpriority56Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h60"]
pub mod intpriority_56;
#[doc = "INTPRIORITY_57 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h61\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_57`]
module"]
#[doc(alias = "INTPRIORITY_57")]
pub type Intpriority57 = crate::Reg<intpriority_57::Intpriority57Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h61"]
pub mod intpriority_57;
#[doc = "INTPRIORITY_58 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h62\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_58`]
module"]
#[doc(alias = "INTPRIORITY_58")]
pub type Intpriority58 = crate::Reg<intpriority_58::Intpriority58Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h62"]
pub mod intpriority_58;
#[doc = "INTPRIORITY_59 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h63\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_59`]
module"]
#[doc(alias = "INTPRIORITY_59")]
pub type Intpriority59 = crate::Reg<intpriority_59::Intpriority59Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h63"]
pub mod intpriority_59;
#[doc = "INTPRIORITY_60 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h64\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_60`]
module"]
#[doc(alias = "INTPRIORITY_60")]
pub type Intpriority60 = crate::Reg<intpriority_60::Intpriority60Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h64"]
pub mod intpriority_60;
#[doc = "INTPRIORITY_61 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h65\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_61`]
module"]
#[doc(alias = "INTPRIORITY_61")]
pub type Intpriority61 = crate::Reg<intpriority_61::Intpriority61Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h65"]
pub mod intpriority_61;
#[doc = "INTPRIORITY_62 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h66\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_62`]
module"]
#[doc(alias = "INTPRIORITY_62")]
pub type Intpriority62 = crate::Reg<intpriority_62::Intpriority62Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h66"]
pub mod intpriority_62;
#[doc = "INTPRIORITY_63 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h67\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_63`]
module"]
#[doc(alias = "INTPRIORITY_63")]
pub type Intpriority63 = crate::Reg<intpriority_63::Intpriority63Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h67"]
pub mod intpriority_63;
#[doc = "INTPRIORITY_64 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h68\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_64`]
module"]
#[doc(alias = "INTPRIORITY_64")]
pub type Intpriority64 = crate::Reg<intpriority_64::Intpriority64Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h68"]
pub mod intpriority_64;
#[doc = "INTPRIORITY_65 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h69\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_65`]
module"]
#[doc(alias = "INTPRIORITY_65")]
pub type Intpriority65 = crate::Reg<intpriority_65::Intpriority65Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h69"]
pub mod intpriority_65;
#[doc = "INTPRIORITY_66 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h70\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_66`]
module"]
#[doc(alias = "INTPRIORITY_66")]
pub type Intpriority66 = crate::Reg<intpriority_66::Intpriority66Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h70"]
pub mod intpriority_66;
#[doc = "INTPRIORITY_67 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h71\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_67`]
module"]
#[doc(alias = "INTPRIORITY_67")]
pub type Intpriority67 = crate::Reg<intpriority_67::Intpriority67Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h71"]
pub mod intpriority_67;
#[doc = "INTPRIORITY_68 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h72\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_68`]
module"]
#[doc(alias = "INTPRIORITY_68")]
pub type Intpriority68 = crate::Reg<intpriority_68::Intpriority68Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h72"]
pub mod intpriority_68;
#[doc = "INTPRIORITY_69 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h73\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_69`]
module"]
#[doc(alias = "INTPRIORITY_69")]
pub type Intpriority69 = crate::Reg<intpriority_69::Intpriority69Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h73"]
pub mod intpriority_69;
#[doc = "INTPRIORITY_70 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h74\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_70`]
module"]
#[doc(alias = "INTPRIORITY_70")]
pub type Intpriority70 = crate::Reg<intpriority_70::Intpriority70Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h74"]
pub mod intpriority_70;
#[doc = "INTPRIORITY_71 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h75\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_71`]
module"]
#[doc(alias = "INTPRIORITY_71")]
pub type Intpriority71 = crate::Reg<intpriority_71::Intpriority71Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h75"]
pub mod intpriority_71;
#[doc = "INTPRIORITY_72 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h76\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_72`]
module"]
#[doc(alias = "INTPRIORITY_72")]
pub type Intpriority72 = crate::Reg<intpriority_72::Intpriority72Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h76"]
pub mod intpriority_72;
#[doc = "INTPRIORITY_73 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h77\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_73`]
module"]
#[doc(alias = "INTPRIORITY_73")]
pub type Intpriority73 = crate::Reg<intpriority_73::Intpriority73Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h77"]
pub mod intpriority_73;
#[doc = "INTPRIORITY_74 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h78\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_74`]
module"]
#[doc(alias = "INTPRIORITY_74")]
pub type Intpriority74 = crate::Reg<intpriority_74::Intpriority74Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h78"]
pub mod intpriority_74;
#[doc = "INTPRIORITY_75 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h79\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_75`]
module"]
#[doc(alias = "INTPRIORITY_75")]
pub type Intpriority75 = crate::Reg<intpriority_75::Intpriority75Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h79"]
pub mod intpriority_75;
#[doc = "INTPRIORITY_76 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h80\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_76`]
module"]
#[doc(alias = "INTPRIORITY_76")]
pub type Intpriority76 = crate::Reg<intpriority_76::Intpriority76Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h80"]
pub mod intpriority_76;
#[doc = "INTPRIORITY_77 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h81\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_77`]
module"]
#[doc(alias = "INTPRIORITY_77")]
pub type Intpriority77 = crate::Reg<intpriority_77::Intpriority77Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h81"]
pub mod intpriority_77;
#[doc = "INTPRIORITY_78 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h82\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_78`]
module"]
#[doc(alias = "INTPRIORITY_78")]
pub type Intpriority78 = crate::Reg<intpriority_78::Intpriority78Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h82"]
pub mod intpriority_78;
#[doc = "INTPRIORITY_79 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h83\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_79`]
module"]
#[doc(alias = "INTPRIORITY_79")]
pub type Intpriority79 = crate::Reg<intpriority_79::Intpriority79Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h83"]
pub mod intpriority_79;
#[doc = "INTPRIORITY_80 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h84\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_80`]
module"]
#[doc(alias = "INTPRIORITY_80")]
pub type Intpriority80 = crate::Reg<intpriority_80::Intpriority80Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h84"]
pub mod intpriority_80;
#[doc = "INTPRIORITY_81 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h85\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_81`]
module"]
#[doc(alias = "INTPRIORITY_81")]
pub type Intpriority81 = crate::Reg<intpriority_81::Intpriority81Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h85"]
pub mod intpriority_81;
#[doc = "INTPRIORITY_82 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h86\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_82`]
module"]
#[doc(alias = "INTPRIORITY_82")]
pub type Intpriority82 = crate::Reg<intpriority_82::Intpriority82Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h86"]
pub mod intpriority_82;
#[doc = "INTPRIORITY_83 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h87\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_83`]
module"]
#[doc(alias = "INTPRIORITY_83")]
pub type Intpriority83 = crate::Reg<intpriority_83::Intpriority83Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h87"]
pub mod intpriority_83;
#[doc = "INTPRIORITY_84 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h88\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_84`]
module"]
#[doc(alias = "INTPRIORITY_84")]
pub type Intpriority84 = crate::Reg<intpriority_84::Intpriority84Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h88"]
pub mod intpriority_84;
#[doc = "INTPRIORITY_85 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h89\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_85`]
module"]
#[doc(alias = "INTPRIORITY_85")]
pub type Intpriority85 = crate::Reg<intpriority_85::Intpriority85Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h89"]
pub mod intpriority_85;
#[doc = "INTPRIORITY_86 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h90\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_86`]
module"]
#[doc(alias = "INTPRIORITY_86")]
pub type Intpriority86 = crate::Reg<intpriority_86::Intpriority86Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h90"]
pub mod intpriority_86;
#[doc = "INTPRIORITY_87 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h91\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_87`]
module"]
#[doc(alias = "INTPRIORITY_87")]
pub type Intpriority87 = crate::Reg<intpriority_87::Intpriority87Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h91"]
pub mod intpriority_87;
#[doc = "INTPRIORITY_88 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h92\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_88`]
module"]
#[doc(alias = "INTPRIORITY_88")]
pub type Intpriority88 = crate::Reg<intpriority_88::Intpriority88Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h92"]
pub mod intpriority_88;
#[doc = "INTPRIORITY_89 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h93\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_89`]
module"]
#[doc(alias = "INTPRIORITY_89")]
pub type Intpriority89 = crate::Reg<intpriority_89::Intpriority89Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h93"]
pub mod intpriority_89;
#[doc = "INTPRIORITY_90 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h94\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_90`]
module"]
#[doc(alias = "INTPRIORITY_90")]
pub type Intpriority90 = crate::Reg<intpriority_90::Intpriority90Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h94"]
pub mod intpriority_90;
#[doc = "INTPRIORITY_91 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h95\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_91`]
module"]
#[doc(alias = "INTPRIORITY_91")]
pub type Intpriority91 = crate::Reg<intpriority_91::Intpriority91Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h95"]
pub mod intpriority_91;
#[doc = "INTPRIORITY_92 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h96\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_92`]
module"]
#[doc(alias = "INTPRIORITY_92")]
pub type Intpriority92 = crate::Reg<intpriority_92::Intpriority92Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h96"]
pub mod intpriority_92;
#[doc = "INTPRIORITY_93 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h97\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_93`]
module"]
#[doc(alias = "INTPRIORITY_93")]
pub type Intpriority93 = crate::Reg<intpriority_93::Intpriority93Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h97"]
pub mod intpriority_93;
#[doc = "INTPRIORITY_94 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h98\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_94`]
module"]
#[doc(alias = "INTPRIORITY_94")]
pub type Intpriority94 = crate::Reg<intpriority_94::Intpriority94Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h98"]
pub mod intpriority_94;
#[doc = "INTPRIORITY_95 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h99\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_95`]
module"]
#[doc(alias = "INTPRIORITY_95")]
pub type Intpriority95 = crate::Reg<intpriority_95::Intpriority95Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h99"]
pub mod intpriority_95;
#[doc = "INTPRIORITY_96 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h100\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_96`]
module"]
#[doc(alias = "INTPRIORITY_96")]
pub type Intpriority96 = crate::Reg<intpriority_96::Intpriority96Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h100"]
pub mod intpriority_96;
#[doc = "INTPRIORITY_97 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h101\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_97`]
module"]
#[doc(alias = "INTPRIORITY_97")]
pub type Intpriority97 = crate::Reg<intpriority_97::Intpriority97Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h101"]
pub mod intpriority_97;
#[doc = "INTPRIORITY_98 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h102\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_98`]
module"]
#[doc(alias = "INTPRIORITY_98")]
pub type Intpriority98 = crate::Reg<intpriority_98::Intpriority98Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h102"]
pub mod intpriority_98;
#[doc = "INTPRIORITY_99 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h103\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_99`]
module"]
#[doc(alias = "INTPRIORITY_99")]
pub type Intpriority99 = crate::Reg<intpriority_99::Intpriority99Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h103"]
pub mod intpriority_99;
#[doc = "INTPRIORITY_100 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h104\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_100`]
module"]
#[doc(alias = "INTPRIORITY_100")]
pub type Intpriority100 = crate::Reg<intpriority_100::Intpriority100Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h104"]
pub mod intpriority_100;
#[doc = "INTPRIORITY_101 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h105\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_101::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_101`]
module"]
#[doc(alias = "INTPRIORITY_101")]
pub type Intpriority101 = crate::Reg<intpriority_101::Intpriority101Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h105"]
pub mod intpriority_101;
#[doc = "INTPRIORITY_102 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h106\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_102::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_102`]
module"]
#[doc(alias = "INTPRIORITY_102")]
pub type Intpriority102 = crate::Reg<intpriority_102::Intpriority102Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h106"]
pub mod intpriority_102;
#[doc = "INTPRIORITY_103 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h107\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_103::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_103::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_103`]
module"]
#[doc(alias = "INTPRIORITY_103")]
pub type Intpriority103 = crate::Reg<intpriority_103::Intpriority103Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h107"]
pub mod intpriority_103;
#[doc = "INTPRIORITY_104 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h108\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_104`]
module"]
#[doc(alias = "INTPRIORITY_104")]
pub type Intpriority104 = crate::Reg<intpriority_104::Intpriority104Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h108"]
pub mod intpriority_104;
#[doc = "INTPRIORITY_105 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h109\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_105::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_105::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_105`]
module"]
#[doc(alias = "INTPRIORITY_105")]
pub type Intpriority105 = crate::Reg<intpriority_105::Intpriority105Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h109"]
pub mod intpriority_105;
#[doc = "INTPRIORITY_106 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h110\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_106::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_106::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_106`]
module"]
#[doc(alias = "INTPRIORITY_106")]
pub type Intpriority106 = crate::Reg<intpriority_106::Intpriority106Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h110"]
pub mod intpriority_106;
#[doc = "INTPRIORITY_107 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h111\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_107::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_107::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_107`]
module"]
#[doc(alias = "INTPRIORITY_107")]
pub type Intpriority107 = crate::Reg<intpriority_107::Intpriority107Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h111"]
pub mod intpriority_107;
#[doc = "INTPRIORITY_108 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h112\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_108`]
module"]
#[doc(alias = "INTPRIORITY_108")]
pub type Intpriority108 = crate::Reg<intpriority_108::Intpriority108Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h112"]
pub mod intpriority_108;
#[doc = "INTPRIORITY_109 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h113\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_109::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_109::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_109`]
module"]
#[doc(alias = "INTPRIORITY_109")]
pub type Intpriority109 = crate::Reg<intpriority_109::Intpriority109Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h113"]
pub mod intpriority_109;
#[doc = "INTPRIORITY_110 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h114\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_110`]
module"]
#[doc(alias = "INTPRIORITY_110")]
pub type Intpriority110 = crate::Reg<intpriority_110::Intpriority110Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h114"]
pub mod intpriority_110;
#[doc = "INTPRIORITY_111 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h115\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_111`]
module"]
#[doc(alias = "INTPRIORITY_111")]
pub type Intpriority111 = crate::Reg<intpriority_111::Intpriority111Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h115"]
pub mod intpriority_111;
#[doc = "INTPRIORITY_112 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h116\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_112`]
module"]
#[doc(alias = "INTPRIORITY_112")]
pub type Intpriority112 = crate::Reg<intpriority_112::Intpriority112Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h116"]
pub mod intpriority_112;
#[doc = "INTPRIORITY_113 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h117\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_113`]
module"]
#[doc(alias = "INTPRIORITY_113")]
pub type Intpriority113 = crate::Reg<intpriority_113::Intpriority113Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h117"]
pub mod intpriority_113;
#[doc = "INTPRIORITY_114 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h118\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_114`]
module"]
#[doc(alias = "INTPRIORITY_114")]
pub type Intpriority114 = crate::Reg<intpriority_114::Intpriority114Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h118"]
pub mod intpriority_114;
#[doc = "INTPRIORITY_115 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h119\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_115`]
module"]
#[doc(alias = "INTPRIORITY_115")]
pub type Intpriority115 = crate::Reg<intpriority_115::Intpriority115Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h119"]
pub mod intpriority_115;
#[doc = "INTPRIORITY_116 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h120\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_116`]
module"]
#[doc(alias = "INTPRIORITY_116")]
pub type Intpriority116 = crate::Reg<intpriority_116::Intpriority116Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h120"]
pub mod intpriority_116;
#[doc = "INTPRIORITY_117 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h121\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_117`]
module"]
#[doc(alias = "INTPRIORITY_117")]
pub type Intpriority117 = crate::Reg<intpriority_117::Intpriority117Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h121"]
pub mod intpriority_117;
#[doc = "INTPRIORITY_118 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h122\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_118`]
module"]
#[doc(alias = "INTPRIORITY_118")]
pub type Intpriority118 = crate::Reg<intpriority_118::Intpriority118Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h122"]
pub mod intpriority_118;
#[doc = "INTPRIORITY_119 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h123\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_119`]
module"]
#[doc(alias = "INTPRIORITY_119")]
pub type Intpriority119 = crate::Reg<intpriority_119::Intpriority119Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h123"]
pub mod intpriority_119;
#[doc = "INTPRIORITY_120 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h124\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_120`]
module"]
#[doc(alias = "INTPRIORITY_120")]
pub type Intpriority120 = crate::Reg<intpriority_120::Intpriority120Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h124"]
pub mod intpriority_120;
#[doc = "INTPRIORITY_121 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h125\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_121`]
module"]
#[doc(alias = "INTPRIORITY_121")]
pub type Intpriority121 = crate::Reg<intpriority_121::Intpriority121Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h125"]
pub mod intpriority_121;
#[doc = "INTPRIORITY_122 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h126\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_122`]
module"]
#[doc(alias = "INTPRIORITY_122")]
pub type Intpriority122 = crate::Reg<intpriority_122::Intpriority122Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h126"]
pub mod intpriority_122;
#[doc = "INTPRIORITY_123 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h127\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_123`]
module"]
#[doc(alias = "INTPRIORITY_123")]
pub type Intpriority123 = crate::Reg<intpriority_123::Intpriority123Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h127"]
pub mod intpriority_123;
#[doc = "INTPRIORITY_124 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h128\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_124`]
module"]
#[doc(alias = "INTPRIORITY_124")]
pub type Intpriority124 = crate::Reg<intpriority_124::Intpriority124Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h128"]
pub mod intpriority_124;
#[doc = "INTPRIORITY_125 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h129\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_125`]
module"]
#[doc(alias = "INTPRIORITY_125")]
pub type Intpriority125 = crate::Reg<intpriority_125::Intpriority125Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h129"]
pub mod intpriority_125;
#[doc = "INTPRIORITY_126 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h130\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_126`]
module"]
#[doc(alias = "INTPRIORITY_126")]
pub type Intpriority126 = crate::Reg<intpriority_126::Intpriority126Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h130"]
pub mod intpriority_126;
#[doc = "INTPRIORITY_127 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h131\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_127`]
module"]
#[doc(alias = "INTPRIORITY_127")]
pub type Intpriority127 = crate::Reg<intpriority_127::Intpriority127Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h131"]
pub mod intpriority_127;
#[doc = "INTPRIORITY_128 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h132\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_128`]
module"]
#[doc(alias = "INTPRIORITY_128")]
pub type Intpriority128 = crate::Reg<intpriority_128::Intpriority128Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h132"]
pub mod intpriority_128;
#[doc = "INTPRIORITY_129 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h133\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_129::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_129::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_129`]
module"]
#[doc(alias = "INTPRIORITY_129")]
pub type Intpriority129 = crate::Reg<intpriority_129::Intpriority129Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h133"]
pub mod intpriority_129;
#[doc = "INTPRIORITY_130 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h134\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_130`]
module"]
#[doc(alias = "INTPRIORITY_130")]
pub type Intpriority130 = crate::Reg<intpriority_130::Intpriority130Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h134"]
pub mod intpriority_130;
#[doc = "INTPRIORITY_131 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h135\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_131::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_131::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_131`]
module"]
#[doc(alias = "INTPRIORITY_131")]
pub type Intpriority131 = crate::Reg<intpriority_131::Intpriority131Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h135"]
pub mod intpriority_131;
#[doc = "INTPRIORITY_132 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h136\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_132::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_132::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_132`]
module"]
#[doc(alias = "INTPRIORITY_132")]
pub type Intpriority132 = crate::Reg<intpriority_132::Intpriority132Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h136"]
pub mod intpriority_132;
#[doc = "INTPRIORITY_133 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h137\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_133::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_133::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_133`]
module"]
#[doc(alias = "INTPRIORITY_133")]
pub type Intpriority133 = crate::Reg<intpriority_133::Intpriority133Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h137"]
pub mod intpriority_133;
#[doc = "INTPRIORITY_134 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h138\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_134`]
module"]
#[doc(alias = "INTPRIORITY_134")]
pub type Intpriority134 = crate::Reg<intpriority_134::Intpriority134Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h138"]
pub mod intpriority_134;
#[doc = "INTPRIORITY_135 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h139\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_135::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_135::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_135`]
module"]
#[doc(alias = "INTPRIORITY_135")]
pub type Intpriority135 = crate::Reg<intpriority_135::Intpriority135Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h139"]
pub mod intpriority_135;
#[doc = "INTPRIORITY_136 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h140\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_136::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_136::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_136`]
module"]
#[doc(alias = "INTPRIORITY_136")]
pub type Intpriority136 = crate::Reg<intpriority_136::Intpriority136Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h140"]
pub mod intpriority_136;
#[doc = "INTPRIORITY_137 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h141\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_137::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_137::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_137`]
module"]
#[doc(alias = "INTPRIORITY_137")]
pub type Intpriority137 = crate::Reg<intpriority_137::Intpriority137Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h141"]
pub mod intpriority_137;
#[doc = "INTPRIORITY_138 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h142\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_138`]
module"]
#[doc(alias = "INTPRIORITY_138")]
pub type Intpriority138 = crate::Reg<intpriority_138::Intpriority138Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h142"]
pub mod intpriority_138;
#[doc = "INTPRIORITY_139 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h143\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_139::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_139::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_139`]
module"]
#[doc(alias = "INTPRIORITY_139")]
pub type Intpriority139 = crate::Reg<intpriority_139::Intpriority139Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h143"]
pub mod intpriority_139;
#[doc = "INTPRIORITY_140 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h144\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_140`]
module"]
#[doc(alias = "INTPRIORITY_140")]
pub type Intpriority140 = crate::Reg<intpriority_140::Intpriority140Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h144"]
pub mod intpriority_140;
#[doc = "INTPRIORITY_141 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h145\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_141::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_141::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_141`]
module"]
#[doc(alias = "INTPRIORITY_141")]
pub type Intpriority141 = crate::Reg<intpriority_141::Intpriority141Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h145"]
pub mod intpriority_141;
#[doc = "INTPRIORITY_142 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h146\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_142::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_142::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_142`]
module"]
#[doc(alias = "INTPRIORITY_142")]
pub type Intpriority142 = crate::Reg<intpriority_142::Intpriority142Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h146"]
pub mod intpriority_142;
#[doc = "INTPRIORITY_143 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h147\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_143::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_143::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_143`]
module"]
#[doc(alias = "INTPRIORITY_143")]
pub type Intpriority143 = crate::Reg<intpriority_143::Intpriority143Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h147"]
pub mod intpriority_143;
#[doc = "INTPRIORITY_144 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h148\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_144`]
module"]
#[doc(alias = "INTPRIORITY_144")]
pub type Intpriority144 = crate::Reg<intpriority_144::Intpriority144Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h148"]
pub mod intpriority_144;
#[doc = "INTPRIORITY_145 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h149\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_145::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_145::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_145`]
module"]
#[doc(alias = "INTPRIORITY_145")]
pub type Intpriority145 = crate::Reg<intpriority_145::Intpriority145Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h149"]
pub mod intpriority_145;
#[doc = "INTPRIORITY_146 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h150\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_146::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_146::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_146`]
module"]
#[doc(alias = "INTPRIORITY_146")]
pub type Intpriority146 = crate::Reg<intpriority_146::Intpriority146Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h150"]
pub mod intpriority_146;
#[doc = "INTPRIORITY_147 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h151\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_147::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_147::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_147`]
module"]
#[doc(alias = "INTPRIORITY_147")]
pub type Intpriority147 = crate::Reg<intpriority_147::Intpriority147Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h151"]
pub mod intpriority_147;
#[doc = "INTPRIORITY_148 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h152\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_148`]
module"]
#[doc(alias = "INTPRIORITY_148")]
pub type Intpriority148 = crate::Reg<intpriority_148::Intpriority148Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h152"]
pub mod intpriority_148;
#[doc = "INTPRIORITY_149 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h153\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_149::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_149::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_149`]
module"]
#[doc(alias = "INTPRIORITY_149")]
pub type Intpriority149 = crate::Reg<intpriority_149::Intpriority149Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h153"]
pub mod intpriority_149;
#[doc = "INTPRIORITY_150 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h154\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_150`]
module"]
#[doc(alias = "INTPRIORITY_150")]
pub type Intpriority150 = crate::Reg<intpriority_150::Intpriority150Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h154"]
pub mod intpriority_150;
#[doc = "INTPRIORITY_151 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h155\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_151::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_151::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_151`]
module"]
#[doc(alias = "INTPRIORITY_151")]
pub type Intpriority151 = crate::Reg<intpriority_151::Intpriority151Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h155"]
pub mod intpriority_151;
#[doc = "INTPRIORITY_152 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h156\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_152::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_152::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_152`]
module"]
#[doc(alias = "INTPRIORITY_152")]
pub type Intpriority152 = crate::Reg<intpriority_152::Intpriority152Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h156"]
pub mod intpriority_152;
#[doc = "INTPRIORITY_153 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h157\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_153::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_153::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_153`]
module"]
#[doc(alias = "INTPRIORITY_153")]
pub type Intpriority153 = crate::Reg<intpriority_153::Intpriority153Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h157"]
pub mod intpriority_153;
#[doc = "INTPRIORITY_154 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h158\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_154`]
module"]
#[doc(alias = "INTPRIORITY_154")]
pub type Intpriority154 = crate::Reg<intpriority_154::Intpriority154Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h158"]
pub mod intpriority_154;
#[doc = "INTPRIORITY_155 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h159\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_155::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_155::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_155`]
module"]
#[doc(alias = "INTPRIORITY_155")]
pub type Intpriority155 = crate::Reg<intpriority_155::Intpriority155Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h159"]
pub mod intpriority_155;
#[doc = "INTPRIORITY_156 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h160\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_156::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_156::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_156`]
module"]
#[doc(alias = "INTPRIORITY_156")]
pub type Intpriority156 = crate::Reg<intpriority_156::Intpriority156Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h160"]
pub mod intpriority_156;
#[doc = "INTPRIORITY_157 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h161\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_157::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_157::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_157`]
module"]
#[doc(alias = "INTPRIORITY_157")]
pub type Intpriority157 = crate::Reg<intpriority_157::Intpriority157Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h161"]
pub mod intpriority_157;
#[doc = "INTPRIORITY_158 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h162\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_158`]
module"]
#[doc(alias = "INTPRIORITY_158")]
pub type Intpriority158 = crate::Reg<intpriority_158::Intpriority158Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h162"]
pub mod intpriority_158;
#[doc = "INTPRIORITY_159 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h163\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_159::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_159::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_159`]
module"]
#[doc(alias = "INTPRIORITY_159")]
pub type Intpriority159 = crate::Reg<intpriority_159::Intpriority159Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h163"]
pub mod intpriority_159;
#[doc = "INTPRIORITY_160 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h164\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_160`]
module"]
#[doc(alias = "INTPRIORITY_160")]
pub type Intpriority160 = crate::Reg<intpriority_160::Intpriority160Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h164"]
pub mod intpriority_160;
#[doc = "INTPRIORITY_161 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h165\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_161::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_161::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_161`]
module"]
#[doc(alias = "INTPRIORITY_161")]
pub type Intpriority161 = crate::Reg<intpriority_161::Intpriority161Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h165"]
pub mod intpriority_161;
#[doc = "INTPRIORITY_162 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h166\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_162::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_162::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_162`]
module"]
#[doc(alias = "INTPRIORITY_162")]
pub type Intpriority162 = crate::Reg<intpriority_162::Intpriority162Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h166"]
pub mod intpriority_162;
#[doc = "INTPRIORITY_163 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h167\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_163::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_163::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_163`]
module"]
#[doc(alias = "INTPRIORITY_163")]
pub type Intpriority163 = crate::Reg<intpriority_163::Intpriority163Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h167"]
pub mod intpriority_163;
#[doc = "INTPRIORITY_164 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h168\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_164`]
module"]
#[doc(alias = "INTPRIORITY_164")]
pub type Intpriority164 = crate::Reg<intpriority_164::Intpriority164Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h168"]
pub mod intpriority_164;
#[doc = "INTPRIORITY_165 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h169\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_165::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_165::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_165`]
module"]
#[doc(alias = "INTPRIORITY_165")]
pub type Intpriority165 = crate::Reg<intpriority_165::Intpriority165Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h169"]
pub mod intpriority_165;
#[doc = "INTPRIORITY_166 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h170\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_166::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_166::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_166`]
module"]
#[doc(alias = "INTPRIORITY_166")]
pub type Intpriority166 = crate::Reg<intpriority_166::Intpriority166Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h170"]
pub mod intpriority_166;
#[doc = "INTPRIORITY_167 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h171\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_167::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_167::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_167`]
module"]
#[doc(alias = "INTPRIORITY_167")]
pub type Intpriority167 = crate::Reg<intpriority_167::Intpriority167Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h171"]
pub mod intpriority_167;
#[doc = "INTPRIORITY_168 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h172\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_168`]
module"]
#[doc(alias = "INTPRIORITY_168")]
pub type Intpriority168 = crate::Reg<intpriority_168::Intpriority168Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h172"]
pub mod intpriority_168;
#[doc = "INTPRIORITY_169 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h173\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_169::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_169::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_169`]
module"]
#[doc(alias = "INTPRIORITY_169")]
pub type Intpriority169 = crate::Reg<intpriority_169::Intpriority169Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h173"]
pub mod intpriority_169;
#[doc = "INTPRIORITY_170 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h174\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_170`]
module"]
#[doc(alias = "INTPRIORITY_170")]
pub type Intpriority170 = crate::Reg<intpriority_170::Intpriority170Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h174"]
pub mod intpriority_170;
#[doc = "INTPRIORITY_171 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h175\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_171::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_171::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_171`]
module"]
#[doc(alias = "INTPRIORITY_171")]
pub type Intpriority171 = crate::Reg<intpriority_171::Intpriority171Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h175"]
pub mod intpriority_171;
#[doc = "INTPRIORITY_172 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h176\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_172::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_172::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_172`]
module"]
#[doc(alias = "INTPRIORITY_172")]
pub type Intpriority172 = crate::Reg<intpriority_172::Intpriority172Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h176"]
pub mod intpriority_172;
#[doc = "INTPRIORITY_173 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h177\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_173::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_173::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_173`]
module"]
#[doc(alias = "INTPRIORITY_173")]
pub type Intpriority173 = crate::Reg<intpriority_173::Intpriority173Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h177"]
pub mod intpriority_173;
#[doc = "INTPRIORITY_174 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h178\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_174`]
module"]
#[doc(alias = "INTPRIORITY_174")]
pub type Intpriority174 = crate::Reg<intpriority_174::Intpriority174Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h178"]
pub mod intpriority_174;
#[doc = "INTPRIORITY_175 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h179\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_175::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_175::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_175`]
module"]
#[doc(alias = "INTPRIORITY_175")]
pub type Intpriority175 = crate::Reg<intpriority_175::Intpriority175Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h179"]
pub mod intpriority_175;
#[doc = "INTPRIORITY_176 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h180\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_176::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_176::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_176`]
module"]
#[doc(alias = "INTPRIORITY_176")]
pub type Intpriority176 = crate::Reg<intpriority_176::Intpriority176Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h180"]
pub mod intpriority_176;
#[doc = "INTPRIORITY_177 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h181\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_177::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_177::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_177`]
module"]
#[doc(alias = "INTPRIORITY_177")]
pub type Intpriority177 = crate::Reg<intpriority_177::Intpriority177Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h181"]
pub mod intpriority_177;
#[doc = "INTPRIORITY_178 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h182\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_178`]
module"]
#[doc(alias = "INTPRIORITY_178")]
pub type Intpriority178 = crate::Reg<intpriority_178::Intpriority178Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h182"]
pub mod intpriority_178;
#[doc = "INTPRIORITY_179 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h183\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_179::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_179::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_179`]
module"]
#[doc(alias = "INTPRIORITY_179")]
pub type Intpriority179 = crate::Reg<intpriority_179::Intpriority179Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h183"]
pub mod intpriority_179;
#[doc = "INTPRIORITY_180 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h184\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_180`]
module"]
#[doc(alias = "INTPRIORITY_180")]
pub type Intpriority180 = crate::Reg<intpriority_180::Intpriority180Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h184"]
pub mod intpriority_180;
#[doc = "INTPRIORITY_181 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h185\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_181::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_181::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_181`]
module"]
#[doc(alias = "INTPRIORITY_181")]
pub type Intpriority181 = crate::Reg<intpriority_181::Intpriority181Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h185"]
pub mod intpriority_181;
#[doc = "INTPRIORITY_182 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h186\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_182::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_182::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_182`]
module"]
#[doc(alias = "INTPRIORITY_182")]
pub type Intpriority182 = crate::Reg<intpriority_182::Intpriority182Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h186"]
pub mod intpriority_182;
#[doc = "INTPRIORITY_183 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h187\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_183::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_183::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_183`]
module"]
#[doc(alias = "INTPRIORITY_183")]
pub type Intpriority183 = crate::Reg<intpriority_183::Intpriority183Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h187"]
pub mod intpriority_183;
#[doc = "INTPRIORITY_184 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h188\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_184`]
module"]
#[doc(alias = "INTPRIORITY_184")]
pub type Intpriority184 = crate::Reg<intpriority_184::Intpriority184Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h188"]
pub mod intpriority_184;
#[doc = "INTPRIORITY_185 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h189\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_185::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_185::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_185`]
module"]
#[doc(alias = "INTPRIORITY_185")]
pub type Intpriority185 = crate::Reg<intpriority_185::Intpriority185Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h189"]
pub mod intpriority_185;
#[doc = "INTPRIORITY_186 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h190\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_186::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_186::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_186`]
module"]
#[doc(alias = "INTPRIORITY_186")]
pub type Intpriority186 = crate::Reg<intpriority_186::Intpriority186Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h190"]
pub mod intpriority_186;
#[doc = "INTPRIORITY_187 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h191\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_187::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_187::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_187`]
module"]
#[doc(alias = "INTPRIORITY_187")]
pub type Intpriority187 = crate::Reg<intpriority_187::Intpriority187Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h191"]
pub mod intpriority_187;
#[doc = "INTPRIORITY_188 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h192\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_188`]
module"]
#[doc(alias = "INTPRIORITY_188")]
pub type Intpriority188 = crate::Reg<intpriority_188::Intpriority188Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h192"]
pub mod intpriority_188;
#[doc = "INTPRIORITY_189 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h193\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_189::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_189::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_189`]
module"]
#[doc(alias = "INTPRIORITY_189")]
pub type Intpriority189 = crate::Reg<intpriority_189::Intpriority189Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h193"]
pub mod intpriority_189;
#[doc = "INTPRIORITY_190 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h194\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_190`]
module"]
#[doc(alias = "INTPRIORITY_190")]
pub type Intpriority190 = crate::Reg<intpriority_190::Intpriority190Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h194"]
pub mod intpriority_190;
#[doc = "INTPRIORITY_191 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h195\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_191::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_191::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_191`]
module"]
#[doc(alias = "INTPRIORITY_191")]
pub type Intpriority191 = crate::Reg<intpriority_191::Intpriority191Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h195"]
pub mod intpriority_191;
#[doc = "INTPRIORITY_192 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h196\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_192::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_192::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_192`]
module"]
#[doc(alias = "INTPRIORITY_192")]
pub type Intpriority192 = crate::Reg<intpriority_192::Intpriority192Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h196"]
pub mod intpriority_192;
#[doc = "INTPRIORITY_193 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h197\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_193::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_193::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_193`]
module"]
#[doc(alias = "INTPRIORITY_193")]
pub type Intpriority193 = crate::Reg<intpriority_193::Intpriority193Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h197"]
pub mod intpriority_193;
#[doc = "INTPRIORITY_194 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h198\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_194`]
module"]
#[doc(alias = "INTPRIORITY_194")]
pub type Intpriority194 = crate::Reg<intpriority_194::Intpriority194Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h198"]
pub mod intpriority_194;
#[doc = "INTPRIORITY_195 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h199\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_195::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_195::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_195`]
module"]
#[doc(alias = "INTPRIORITY_195")]
pub type Intpriority195 = crate::Reg<intpriority_195::Intpriority195Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h199"]
pub mod intpriority_195;
#[doc = "INTPRIORITY_196 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h200\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_196::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_196::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_196`]
module"]
#[doc(alias = "INTPRIORITY_196")]
pub type Intpriority196 = crate::Reg<intpriority_196::Intpriority196Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h200"]
pub mod intpriority_196;
#[doc = "INTPRIORITY_197 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h201\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_197::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_197::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_197`]
module"]
#[doc(alias = "INTPRIORITY_197")]
pub type Intpriority197 = crate::Reg<intpriority_197::Intpriority197Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h201"]
pub mod intpriority_197;
#[doc = "INTPRIORITY_198 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h202\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_198`]
module"]
#[doc(alias = "INTPRIORITY_198")]
pub type Intpriority198 = crate::Reg<intpriority_198::Intpriority198Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h202"]
pub mod intpriority_198;
#[doc = "INTPRIORITY_199 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h203\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_199::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_199::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_199`]
module"]
#[doc(alias = "INTPRIORITY_199")]
pub type Intpriority199 = crate::Reg<intpriority_199::Intpriority199Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h203"]
pub mod intpriority_199;
#[doc = "INTPRIORITY_200 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h204\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_200`]
module"]
#[doc(alias = "INTPRIORITY_200")]
pub type Intpriority200 = crate::Reg<intpriority_200::Intpriority200Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h204"]
pub mod intpriority_200;
#[doc = "INTPRIORITY_201 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h205\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_201::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_201::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_201`]
module"]
#[doc(alias = "INTPRIORITY_201")]
pub type Intpriority201 = crate::Reg<intpriority_201::Intpriority201Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h205"]
pub mod intpriority_201;
#[doc = "INTPRIORITY_202 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h206\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_202::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_202::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_202`]
module"]
#[doc(alias = "INTPRIORITY_202")]
pub type Intpriority202 = crate::Reg<intpriority_202::Intpriority202Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h206"]
pub mod intpriority_202;
#[doc = "INTPRIORITY_203 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h207\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_203::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_203::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_203`]
module"]
#[doc(alias = "INTPRIORITY_203")]
pub type Intpriority203 = crate::Reg<intpriority_203::Intpriority203Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h207"]
pub mod intpriority_203;
#[doc = "INTPRIORITY_204 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h208\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_204`]
module"]
#[doc(alias = "INTPRIORITY_204")]
pub type Intpriority204 = crate::Reg<intpriority_204::Intpriority204Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h208"]
pub mod intpriority_204;
#[doc = "INTPRIORITY_205 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h209\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_205::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_205::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_205`]
module"]
#[doc(alias = "INTPRIORITY_205")]
pub type Intpriority205 = crate::Reg<intpriority_205::Intpriority205Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h209"]
pub mod intpriority_205;
#[doc = "INTPRIORITY_206 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h210\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_206::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_206::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_206`]
module"]
#[doc(alias = "INTPRIORITY_206")]
pub type Intpriority206 = crate::Reg<intpriority_206::Intpriority206Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h210"]
pub mod intpriority_206;
#[doc = "INTPRIORITY_207 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h211\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_207::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_207::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_207`]
module"]
#[doc(alias = "INTPRIORITY_207")]
pub type Intpriority207 = crate::Reg<intpriority_207::Intpriority207Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h211"]
pub mod intpriority_207;
#[doc = "INTPRIORITY_208 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h212\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_208`]
module"]
#[doc(alias = "INTPRIORITY_208")]
pub type Intpriority208 = crate::Reg<intpriority_208::Intpriority208Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h212"]
pub mod intpriority_208;
#[doc = "INTPRIORITY_209 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h213\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_209::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_209::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_209`]
module"]
#[doc(alias = "INTPRIORITY_209")]
pub type Intpriority209 = crate::Reg<intpriority_209::Intpriority209Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h213"]
pub mod intpriority_209;
#[doc = "INTPRIORITY_210 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h214\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_210`]
module"]
#[doc(alias = "INTPRIORITY_210")]
pub type Intpriority210 = crate::Reg<intpriority_210::Intpriority210Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h214"]
pub mod intpriority_210;
#[doc = "INTPRIORITY_211 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h215\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_211::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_211::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_211`]
module"]
#[doc(alias = "INTPRIORITY_211")]
pub type Intpriority211 = crate::Reg<intpriority_211::Intpriority211Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h215"]
pub mod intpriority_211;
#[doc = "INTPRIORITY_212 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h216\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_212::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_212::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_212`]
module"]
#[doc(alias = "INTPRIORITY_212")]
pub type Intpriority212 = crate::Reg<intpriority_212::Intpriority212Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h216"]
pub mod intpriority_212;
#[doc = "INTPRIORITY_213 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h217\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_213::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_213::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_213`]
module"]
#[doc(alias = "INTPRIORITY_213")]
pub type Intpriority213 = crate::Reg<intpriority_213::Intpriority213Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h217"]
pub mod intpriority_213;
#[doc = "INTPRIORITY_214 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h218\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_214`]
module"]
#[doc(alias = "INTPRIORITY_214")]
pub type Intpriority214 = crate::Reg<intpriority_214::Intpriority214Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h218"]
pub mod intpriority_214;
#[doc = "INTPRIORITY_215 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h219\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_215::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_215::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_215`]
module"]
#[doc(alias = "INTPRIORITY_215")]
pub type Intpriority215 = crate::Reg<intpriority_215::Intpriority215Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h219"]
pub mod intpriority_215;
#[doc = "INTPRIORITY_216 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h220\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_216::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_216::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_216`]
module"]
#[doc(alias = "INTPRIORITY_216")]
pub type Intpriority216 = crate::Reg<intpriority_216::Intpriority216Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h220"]
pub mod intpriority_216;
#[doc = "INTPRIORITY_217 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h221\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_217::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_217::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_217`]
module"]
#[doc(alias = "INTPRIORITY_217")]
pub type Intpriority217 = crate::Reg<intpriority_217::Intpriority217Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h221"]
pub mod intpriority_217;
#[doc = "INTPRIORITY_218 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h222\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_218`]
module"]
#[doc(alias = "INTPRIORITY_218")]
pub type Intpriority218 = crate::Reg<intpriority_218::Intpriority218Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h222"]
pub mod intpriority_218;
#[doc = "INTPRIORITY_219 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h223\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_219::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_219::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_219`]
module"]
#[doc(alias = "INTPRIORITY_219")]
pub type Intpriority219 = crate::Reg<intpriority_219::Intpriority219Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h223"]
pub mod intpriority_219;
#[doc = "INTPRIORITY_220 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h224\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_220`]
module"]
#[doc(alias = "INTPRIORITY_220")]
pub type Intpriority220 = crate::Reg<intpriority_220::Intpriority220Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h224"]
pub mod intpriority_220;
#[doc = "INTPRIORITY_221 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h225\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_221::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_221::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_221`]
module"]
#[doc(alias = "INTPRIORITY_221")]
pub type Intpriority221 = crate::Reg<intpriority_221::Intpriority221Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h225"]
pub mod intpriority_221;
#[doc = "INTPRIORITY_222 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h226\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_222::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_222::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_222`]
module"]
#[doc(alias = "INTPRIORITY_222")]
pub type Intpriority222 = crate::Reg<intpriority_222::Intpriority222Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h226"]
pub mod intpriority_222;
#[doc = "INTPRIORITY_223 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h227\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_223::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_223::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_223`]
module"]
#[doc(alias = "INTPRIORITY_223")]
pub type Intpriority223 = crate::Reg<intpriority_223::Intpriority223Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h227"]
pub mod intpriority_223;
#[doc = "INTPRIORITY_224 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h228\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_224`]
module"]
#[doc(alias = "INTPRIORITY_224")]
pub type Intpriority224 = crate::Reg<intpriority_224::Intpriority224Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h228"]
pub mod intpriority_224;
#[doc = "INTPRIORITY_225 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h229\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_225::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_225::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_225`]
module"]
#[doc(alias = "INTPRIORITY_225")]
pub type Intpriority225 = crate::Reg<intpriority_225::Intpriority225Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h229"]
pub mod intpriority_225;
#[doc = "INTPRIORITY_226 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h230\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_226::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_226::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_226`]
module"]
#[doc(alias = "INTPRIORITY_226")]
pub type Intpriority226 = crate::Reg<intpriority_226::Intpriority226Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h230"]
pub mod intpriority_226;
#[doc = "INTPRIORITY_227 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h231\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_227::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_227::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_227`]
module"]
#[doc(alias = "INTPRIORITY_227")]
pub type Intpriority227 = crate::Reg<intpriority_227::Intpriority227Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h231"]
pub mod intpriority_227;
#[doc = "INTPRIORITY_228 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h232\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_228`]
module"]
#[doc(alias = "INTPRIORITY_228")]
pub type Intpriority228 = crate::Reg<intpriority_228::Intpriority228Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h232"]
pub mod intpriority_228;
#[doc = "INTPRIORITY_229 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h233\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_229::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_229::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_229`]
module"]
#[doc(alias = "INTPRIORITY_229")]
pub type Intpriority229 = crate::Reg<intpriority_229::Intpriority229Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h233"]
pub mod intpriority_229;
#[doc = "INTPRIORITY_230 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h234\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_230`]
module"]
#[doc(alias = "INTPRIORITY_230")]
pub type Intpriority230 = crate::Reg<intpriority_230::Intpriority230Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h234"]
pub mod intpriority_230;
#[doc = "INTPRIORITY_231 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h235\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_231::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_231::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_231`]
module"]
#[doc(alias = "INTPRIORITY_231")]
pub type Intpriority231 = crate::Reg<intpriority_231::Intpriority231Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h235"]
pub mod intpriority_231;
#[doc = "INTPRIORITY_232 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h236\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_232::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_232::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_232`]
module"]
#[doc(alias = "INTPRIORITY_232")]
pub type Intpriority232 = crate::Reg<intpriority_232::Intpriority232Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h236"]
pub mod intpriority_232;
#[doc = "INTPRIORITY_233 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h237\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_233::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_233::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_233`]
module"]
#[doc(alias = "INTPRIORITY_233")]
pub type Intpriority233 = crate::Reg<intpriority_233::Intpriority233Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h237"]
pub mod intpriority_233;
#[doc = "INTPRIORITY_234 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h238\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_234`]
module"]
#[doc(alias = "INTPRIORITY_234")]
pub type Intpriority234 = crate::Reg<intpriority_234::Intpriority234Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h238"]
pub mod intpriority_234;
#[doc = "INTPRIORITY_235 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h239\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_235::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_235::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_235`]
module"]
#[doc(alias = "INTPRIORITY_235")]
pub type Intpriority235 = crate::Reg<intpriority_235::Intpriority235Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h239"]
pub mod intpriority_235;
#[doc = "INTPRIORITY_236 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h240\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_236::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_236::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_236`]
module"]
#[doc(alias = "INTPRIORITY_236")]
pub type Intpriority236 = crate::Reg<intpriority_236::Intpriority236Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h240"]
pub mod intpriority_236;
#[doc = "INTPRIORITY_237 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h241\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_237::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_237::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_237`]
module"]
#[doc(alias = "INTPRIORITY_237")]
pub type Intpriority237 = crate::Reg<intpriority_237::Intpriority237Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h241"]
pub mod intpriority_237;
#[doc = "INTPRIORITY_238 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h242\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_238`]
module"]
#[doc(alias = "INTPRIORITY_238")]
pub type Intpriority238 = crate::Reg<intpriority_238::Intpriority238Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h242"]
pub mod intpriority_238;
#[doc = "INTPRIORITY_239 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h243\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_239::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_239::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_239`]
module"]
#[doc(alias = "INTPRIORITY_239")]
pub type Intpriority239 = crate::Reg<intpriority_239::Intpriority239Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h243"]
pub mod intpriority_239;
#[doc = "INTPRIORITY_240 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h244\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_240`]
module"]
#[doc(alias = "INTPRIORITY_240")]
pub type Intpriority240 = crate::Reg<intpriority_240::Intpriority240Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h244"]
pub mod intpriority_240;
#[doc = "INTPRIORITY_241 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h245\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_241::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_241::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_241`]
module"]
#[doc(alias = "INTPRIORITY_241")]
pub type Intpriority241 = crate::Reg<intpriority_241::Intpriority241Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h245"]
pub mod intpriority_241;
#[doc = "INTPRIORITY_242 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h246\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_242::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_242::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_242`]
module"]
#[doc(alias = "INTPRIORITY_242")]
pub type Intpriority242 = crate::Reg<intpriority_242::Intpriority242Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h246"]
pub mod intpriority_242;
#[doc = "INTPRIORITY_243 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h247\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_243::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_243::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_243`]
module"]
#[doc(alias = "INTPRIORITY_243")]
pub type Intpriority243 = crate::Reg<intpriority_243::Intpriority243Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h247"]
pub mod intpriority_243;
#[doc = "INTPRIORITY_244 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h248\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_244::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_244`]
module"]
#[doc(alias = "INTPRIORITY_244")]
pub type Intpriority244 = crate::Reg<intpriority_244::Intpriority244Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h248"]
pub mod intpriority_244;
#[doc = "INTPRIORITY_245 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h249\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_245::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_245::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_245`]
module"]
#[doc(alias = "INTPRIORITY_245")]
pub type Intpriority245 = crate::Reg<intpriority_245::Intpriority245Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h249"]
pub mod intpriority_245;
#[doc = "INTPRIORITY_246 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h250\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_246::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_246::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_246`]
module"]
#[doc(alias = "INTPRIORITY_246")]
pub type Intpriority246 = crate::Reg<intpriority_246::Intpriority246Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h250"]
pub mod intpriority_246;
#[doc = "INTPRIORITY_247 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h251\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_247::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_247::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_247`]
module"]
#[doc(alias = "INTPRIORITY_247")]
pub type Intpriority247 = crate::Reg<intpriority_247::Intpriority247Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h251"]
pub mod intpriority_247;
#[doc = "INTPRIORITY_248 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h252\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_248::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_248`]
module"]
#[doc(alias = "INTPRIORITY_248")]
pub type Intpriority248 = crate::Reg<intpriority_248::Intpriority248Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h252"]
pub mod intpriority_248;
#[doc = "INTPRIORITY_249 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h253\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_249::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_249::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_249`]
module"]
#[doc(alias = "INTPRIORITY_249")]
pub type Intpriority249 = crate::Reg<intpriority_249::Intpriority249Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h253"]
pub mod intpriority_249;
#[doc = "INTPRIORITY_250 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h254\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_250`]
module"]
#[doc(alias = "INTPRIORITY_250")]
pub type Intpriority250 = crate::Reg<intpriority_250::Intpriority250Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h254"]
pub mod intpriority_250;
#[doc = "INTPRIORITY_251 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h255\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_251::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_251::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_251`]
module"]
#[doc(alias = "INTPRIORITY_251")]
pub type Intpriority251 = crate::Reg<intpriority_251::Intpriority251Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h255"]
pub mod intpriority_251;
#[doc = "INTPRIORITY_252 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h256\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_252::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_252::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_252`]
module"]
#[doc(alias = "INTPRIORITY_252")]
pub type Intpriority252 = crate::Reg<intpriority_252::Intpriority252Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h256"]
pub mod intpriority_252;
#[doc = "INTPRIORITY_253 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h257\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_253::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_253::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_253`]
module"]
#[doc(alias = "INTPRIORITY_253")]
pub type Intpriority253 = crate::Reg<intpriority_253::Intpriority253Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h257"]
pub mod intpriority_253;
#[doc = "INTPRIORITY_254 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h258\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_254::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_254`]
module"]
#[doc(alias = "INTPRIORITY_254")]
pub type Intpriority254 = crate::Reg<intpriority_254::Intpriority254Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h258"]
pub mod intpriority_254;
#[doc = "INTPRIORITY_255 (rw) register accessor: Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h259\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_255::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_255::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpriority_255`]
module"]
#[doc(alias = "INTPRIORITY_255")]
pub type Intpriority255 = crate::Reg<intpriority_255::Intpriority255Spec>;
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h259"]
pub mod intpriority_255;
#[doc = "INTVECTOR (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h4\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector`]
module"]
#[doc(alias = "INTVECTOR")]
pub type Intvector = crate::Reg<intvector::IntvectorSpec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h4"]
pub mod intvector;
#[doc = "INTVECTOR_1 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h5\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_1`]
module"]
#[doc(alias = "INTVECTOR_1")]
pub type Intvector1 = crate::Reg<intvector_1::Intvector1Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h5"]
pub mod intvector_1;
#[doc = "INTVECTOR_2 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h6\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_2`]
module"]
#[doc(alias = "INTVECTOR_2")]
pub type Intvector2 = crate::Reg<intvector_2::Intvector2Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h6"]
pub mod intvector_2;
#[doc = "INTVECTOR_3 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h7\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_3`]
module"]
#[doc(alias = "INTVECTOR_3")]
pub type Intvector3 = crate::Reg<intvector_3::Intvector3Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h7"]
pub mod intvector_3;
#[doc = "INTVECTOR_4 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h8\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_4`]
module"]
#[doc(alias = "INTVECTOR_4")]
pub type Intvector4 = crate::Reg<intvector_4::Intvector4Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h8"]
pub mod intvector_4;
#[doc = "INTVECTOR_5 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h9\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_5`]
module"]
#[doc(alias = "INTVECTOR_5")]
pub type Intvector5 = crate::Reg<intvector_5::Intvector5Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h9"]
pub mod intvector_5;
#[doc = "INTVECTOR_6 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h10\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_6`]
module"]
#[doc(alias = "INTVECTOR_6")]
pub type Intvector6 = crate::Reg<intvector_6::Intvector6Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h10"]
pub mod intvector_6;
#[doc = "INTVECTOR_7 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h11\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_7`]
module"]
#[doc(alias = "INTVECTOR_7")]
pub type Intvector7 = crate::Reg<intvector_7::Intvector7Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h11"]
pub mod intvector_7;
#[doc = "INTVECTOR_8 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h12\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_8`]
module"]
#[doc(alias = "INTVECTOR_8")]
pub type Intvector8 = crate::Reg<intvector_8::Intvector8Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h12"]
pub mod intvector_8;
#[doc = "INTVECTOR_9 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h13\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_9`]
module"]
#[doc(alias = "INTVECTOR_9")]
pub type Intvector9 = crate::Reg<intvector_9::Intvector9Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h13"]
pub mod intvector_9;
#[doc = "INTVECTOR_10 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h14\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_10`]
module"]
#[doc(alias = "INTVECTOR_10")]
pub type Intvector10 = crate::Reg<intvector_10::Intvector10Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h14"]
pub mod intvector_10;
#[doc = "INTVECTOR_11 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h15\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_11`]
module"]
#[doc(alias = "INTVECTOR_11")]
pub type Intvector11 = crate::Reg<intvector_11::Intvector11Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h15"]
pub mod intvector_11;
#[doc = "INTVECTOR_12 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h16\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_12`]
module"]
#[doc(alias = "INTVECTOR_12")]
pub type Intvector12 = crate::Reg<intvector_12::Intvector12Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h16"]
pub mod intvector_12;
#[doc = "INTVECTOR_13 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h17\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_13`]
module"]
#[doc(alias = "INTVECTOR_13")]
pub type Intvector13 = crate::Reg<intvector_13::Intvector13Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h17"]
pub mod intvector_13;
#[doc = "INTVECTOR_14 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_14`]
module"]
#[doc(alias = "INTVECTOR_14")]
pub type Intvector14 = crate::Reg<intvector_14::Intvector14Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h18"]
pub mod intvector_14;
#[doc = "INTVECTOR_15 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h19\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_15`]
module"]
#[doc(alias = "INTVECTOR_15")]
pub type Intvector15 = crate::Reg<intvector_15::Intvector15Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h19"]
pub mod intvector_15;
#[doc = "INTVECTOR_16 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h20\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_16`]
module"]
#[doc(alias = "INTVECTOR_16")]
pub type Intvector16 = crate::Reg<intvector_16::Intvector16Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h20"]
pub mod intvector_16;
#[doc = "INTVECTOR_17 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h21\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_17`]
module"]
#[doc(alias = "INTVECTOR_17")]
pub type Intvector17 = crate::Reg<intvector_17::Intvector17Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h21"]
pub mod intvector_17;
#[doc = "INTVECTOR_18 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h22\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_18`]
module"]
#[doc(alias = "INTVECTOR_18")]
pub type Intvector18 = crate::Reg<intvector_18::Intvector18Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h22"]
pub mod intvector_18;
#[doc = "INTVECTOR_19 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h23\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_19`]
module"]
#[doc(alias = "INTVECTOR_19")]
pub type Intvector19 = crate::Reg<intvector_19::Intvector19Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h23"]
pub mod intvector_19;
#[doc = "INTVECTOR_20 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h24\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_20`]
module"]
#[doc(alias = "INTVECTOR_20")]
pub type Intvector20 = crate::Reg<intvector_20::Intvector20Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h24"]
pub mod intvector_20;
#[doc = "INTVECTOR_21 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h25\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_21`]
module"]
#[doc(alias = "INTVECTOR_21")]
pub type Intvector21 = crate::Reg<intvector_21::Intvector21Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h25"]
pub mod intvector_21;
#[doc = "INTVECTOR_22 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h26\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_22`]
module"]
#[doc(alias = "INTVECTOR_22")]
pub type Intvector22 = crate::Reg<intvector_22::Intvector22Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h26"]
pub mod intvector_22;
#[doc = "INTVECTOR_23 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h27\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_23`]
module"]
#[doc(alias = "INTVECTOR_23")]
pub type Intvector23 = crate::Reg<intvector_23::Intvector23Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h27"]
pub mod intvector_23;
#[doc = "INTVECTOR_24 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h28\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_24`]
module"]
#[doc(alias = "INTVECTOR_24")]
pub type Intvector24 = crate::Reg<intvector_24::Intvector24Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h28"]
pub mod intvector_24;
#[doc = "INTVECTOR_25 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h29\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_25`]
module"]
#[doc(alias = "INTVECTOR_25")]
pub type Intvector25 = crate::Reg<intvector_25::Intvector25Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h29"]
pub mod intvector_25;
#[doc = "INTVECTOR_26 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h30\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_26`]
module"]
#[doc(alias = "INTVECTOR_26")]
pub type Intvector26 = crate::Reg<intvector_26::Intvector26Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h30"]
pub mod intvector_26;
#[doc = "INTVECTOR_27 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h31\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_27`]
module"]
#[doc(alias = "INTVECTOR_27")]
pub type Intvector27 = crate::Reg<intvector_27::Intvector27Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h31"]
pub mod intvector_27;
#[doc = "INTVECTOR_28 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h32\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_28`]
module"]
#[doc(alias = "INTVECTOR_28")]
pub type Intvector28 = crate::Reg<intvector_28::Intvector28Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h32"]
pub mod intvector_28;
#[doc = "INTVECTOR_29 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h33\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_29`]
module"]
#[doc(alias = "INTVECTOR_29")]
pub type Intvector29 = crate::Reg<intvector_29::Intvector29Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h33"]
pub mod intvector_29;
#[doc = "INTVECTOR_30 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h34\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_30`]
module"]
#[doc(alias = "INTVECTOR_30")]
pub type Intvector30 = crate::Reg<intvector_30::Intvector30Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h34"]
pub mod intvector_30;
#[doc = "INTVECTOR_31 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h35\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_31`]
module"]
#[doc(alias = "INTVECTOR_31")]
pub type Intvector31 = crate::Reg<intvector_31::Intvector31Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h35"]
pub mod intvector_31;
#[doc = "INTVECTOR_32 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h36\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_32`]
module"]
#[doc(alias = "INTVECTOR_32")]
pub type Intvector32 = crate::Reg<intvector_32::Intvector32Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h36"]
pub mod intvector_32;
#[doc = "INTVECTOR_33 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h37\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_33`]
module"]
#[doc(alias = "INTVECTOR_33")]
pub type Intvector33 = crate::Reg<intvector_33::Intvector33Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h37"]
pub mod intvector_33;
#[doc = "INTVECTOR_34 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h38\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_34`]
module"]
#[doc(alias = "INTVECTOR_34")]
pub type Intvector34 = crate::Reg<intvector_34::Intvector34Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h38"]
pub mod intvector_34;
#[doc = "INTVECTOR_35 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h39\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_35`]
module"]
#[doc(alias = "INTVECTOR_35")]
pub type Intvector35 = crate::Reg<intvector_35::Intvector35Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h39"]
pub mod intvector_35;
#[doc = "INTVECTOR_36 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h40\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_36`]
module"]
#[doc(alias = "INTVECTOR_36")]
pub type Intvector36 = crate::Reg<intvector_36::Intvector36Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h40"]
pub mod intvector_36;
#[doc = "INTVECTOR_37 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h41\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_37`]
module"]
#[doc(alias = "INTVECTOR_37")]
pub type Intvector37 = crate::Reg<intvector_37::Intvector37Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h41"]
pub mod intvector_37;
#[doc = "INTVECTOR_38 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h42\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_38`]
module"]
#[doc(alias = "INTVECTOR_38")]
pub type Intvector38 = crate::Reg<intvector_38::Intvector38Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h42"]
pub mod intvector_38;
#[doc = "INTVECTOR_39 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h43\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_39`]
module"]
#[doc(alias = "INTVECTOR_39")]
pub type Intvector39 = crate::Reg<intvector_39::Intvector39Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h43"]
pub mod intvector_39;
#[doc = "INTVECTOR_40 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h44\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_40`]
module"]
#[doc(alias = "INTVECTOR_40")]
pub type Intvector40 = crate::Reg<intvector_40::Intvector40Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h44"]
pub mod intvector_40;
#[doc = "INTVECTOR_41 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h45\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_41`]
module"]
#[doc(alias = "INTVECTOR_41")]
pub type Intvector41 = crate::Reg<intvector_41::Intvector41Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h45"]
pub mod intvector_41;
#[doc = "INTVECTOR_42 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h46\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_42`]
module"]
#[doc(alias = "INTVECTOR_42")]
pub type Intvector42 = crate::Reg<intvector_42::Intvector42Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h46"]
pub mod intvector_42;
#[doc = "INTVECTOR_43 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h47\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_43`]
module"]
#[doc(alias = "INTVECTOR_43")]
pub type Intvector43 = crate::Reg<intvector_43::Intvector43Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h47"]
pub mod intvector_43;
#[doc = "INTVECTOR_44 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h48\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_44`]
module"]
#[doc(alias = "INTVECTOR_44")]
pub type Intvector44 = crate::Reg<intvector_44::Intvector44Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h48"]
pub mod intvector_44;
#[doc = "INTVECTOR_45 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h49\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_45`]
module"]
#[doc(alias = "INTVECTOR_45")]
pub type Intvector45 = crate::Reg<intvector_45::Intvector45Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h49"]
pub mod intvector_45;
#[doc = "INTVECTOR_46 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h50\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_46`]
module"]
#[doc(alias = "INTVECTOR_46")]
pub type Intvector46 = crate::Reg<intvector_46::Intvector46Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h50"]
pub mod intvector_46;
#[doc = "INTVECTOR_47 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h51\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_47`]
module"]
#[doc(alias = "INTVECTOR_47")]
pub type Intvector47 = crate::Reg<intvector_47::Intvector47Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h51"]
pub mod intvector_47;
#[doc = "INTVECTOR_48 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h52\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_48`]
module"]
#[doc(alias = "INTVECTOR_48")]
pub type Intvector48 = crate::Reg<intvector_48::Intvector48Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h52"]
pub mod intvector_48;
#[doc = "INTVECTOR_49 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h53\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_49`]
module"]
#[doc(alias = "INTVECTOR_49")]
pub type Intvector49 = crate::Reg<intvector_49::Intvector49Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h53"]
pub mod intvector_49;
#[doc = "INTVECTOR_50 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h54\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_50`]
module"]
#[doc(alias = "INTVECTOR_50")]
pub type Intvector50 = crate::Reg<intvector_50::Intvector50Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h54"]
pub mod intvector_50;
#[doc = "INTVECTOR_51 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h55\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_51`]
module"]
#[doc(alias = "INTVECTOR_51")]
pub type Intvector51 = crate::Reg<intvector_51::Intvector51Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h55"]
pub mod intvector_51;
#[doc = "INTVECTOR_52 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h56\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_52`]
module"]
#[doc(alias = "INTVECTOR_52")]
pub type Intvector52 = crate::Reg<intvector_52::Intvector52Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h56"]
pub mod intvector_52;
#[doc = "INTVECTOR_53 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h57\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_53`]
module"]
#[doc(alias = "INTVECTOR_53")]
pub type Intvector53 = crate::Reg<intvector_53::Intvector53Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h57"]
pub mod intvector_53;
#[doc = "INTVECTOR_54 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h58\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_54`]
module"]
#[doc(alias = "INTVECTOR_54")]
pub type Intvector54 = crate::Reg<intvector_54::Intvector54Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h58"]
pub mod intvector_54;
#[doc = "INTVECTOR_55 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h59\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_55`]
module"]
#[doc(alias = "INTVECTOR_55")]
pub type Intvector55 = crate::Reg<intvector_55::Intvector55Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h59"]
pub mod intvector_55;
#[doc = "INTVECTOR_56 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h60\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_56`]
module"]
#[doc(alias = "INTVECTOR_56")]
pub type Intvector56 = crate::Reg<intvector_56::Intvector56Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h60"]
pub mod intvector_56;
#[doc = "INTVECTOR_57 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h61\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_57`]
module"]
#[doc(alias = "INTVECTOR_57")]
pub type Intvector57 = crate::Reg<intvector_57::Intvector57Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h61"]
pub mod intvector_57;
#[doc = "INTVECTOR_58 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h62\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_58`]
module"]
#[doc(alias = "INTVECTOR_58")]
pub type Intvector58 = crate::Reg<intvector_58::Intvector58Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h62"]
pub mod intvector_58;
#[doc = "INTVECTOR_59 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h63\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_59`]
module"]
#[doc(alias = "INTVECTOR_59")]
pub type Intvector59 = crate::Reg<intvector_59::Intvector59Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h63"]
pub mod intvector_59;
#[doc = "INTVECTOR_60 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h64\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_60`]
module"]
#[doc(alias = "INTVECTOR_60")]
pub type Intvector60 = crate::Reg<intvector_60::Intvector60Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h64"]
pub mod intvector_60;
#[doc = "INTVECTOR_61 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h65\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_61`]
module"]
#[doc(alias = "INTVECTOR_61")]
pub type Intvector61 = crate::Reg<intvector_61::Intvector61Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h65"]
pub mod intvector_61;
#[doc = "INTVECTOR_62 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h66\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_62`]
module"]
#[doc(alias = "INTVECTOR_62")]
pub type Intvector62 = crate::Reg<intvector_62::Intvector62Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h66"]
pub mod intvector_62;
#[doc = "INTVECTOR_63 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h67\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_63`]
module"]
#[doc(alias = "INTVECTOR_63")]
pub type Intvector63 = crate::Reg<intvector_63::Intvector63Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h67"]
pub mod intvector_63;
#[doc = "INTVECTOR_64 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h68\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_64`]
module"]
#[doc(alias = "INTVECTOR_64")]
pub type Intvector64 = crate::Reg<intvector_64::Intvector64Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h68"]
pub mod intvector_64;
#[doc = "INTVECTOR_65 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h69\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_65`]
module"]
#[doc(alias = "INTVECTOR_65")]
pub type Intvector65 = crate::Reg<intvector_65::Intvector65Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h69"]
pub mod intvector_65;
#[doc = "INTVECTOR_66 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h70\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_66`]
module"]
#[doc(alias = "INTVECTOR_66")]
pub type Intvector66 = crate::Reg<intvector_66::Intvector66Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h70"]
pub mod intvector_66;
#[doc = "INTVECTOR_67 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h71\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_67`]
module"]
#[doc(alias = "INTVECTOR_67")]
pub type Intvector67 = crate::Reg<intvector_67::Intvector67Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h71"]
pub mod intvector_67;
#[doc = "INTVECTOR_68 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h72\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_68`]
module"]
#[doc(alias = "INTVECTOR_68")]
pub type Intvector68 = crate::Reg<intvector_68::Intvector68Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h72"]
pub mod intvector_68;
#[doc = "INTVECTOR_69 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h73\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_69`]
module"]
#[doc(alias = "INTVECTOR_69")]
pub type Intvector69 = crate::Reg<intvector_69::Intvector69Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h73"]
pub mod intvector_69;
#[doc = "INTVECTOR_70 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h74\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_70`]
module"]
#[doc(alias = "INTVECTOR_70")]
pub type Intvector70 = crate::Reg<intvector_70::Intvector70Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h74"]
pub mod intvector_70;
#[doc = "INTVECTOR_71 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h75\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_71`]
module"]
#[doc(alias = "INTVECTOR_71")]
pub type Intvector71 = crate::Reg<intvector_71::Intvector71Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h75"]
pub mod intvector_71;
#[doc = "INTVECTOR_72 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h76\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_72`]
module"]
#[doc(alias = "INTVECTOR_72")]
pub type Intvector72 = crate::Reg<intvector_72::Intvector72Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h76"]
pub mod intvector_72;
#[doc = "INTVECTOR_73 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h77\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_73`]
module"]
#[doc(alias = "INTVECTOR_73")]
pub type Intvector73 = crate::Reg<intvector_73::Intvector73Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h77"]
pub mod intvector_73;
#[doc = "INTVECTOR_74 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h78\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_74`]
module"]
#[doc(alias = "INTVECTOR_74")]
pub type Intvector74 = crate::Reg<intvector_74::Intvector74Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h78"]
pub mod intvector_74;
#[doc = "INTVECTOR_75 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h79\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_75`]
module"]
#[doc(alias = "INTVECTOR_75")]
pub type Intvector75 = crate::Reg<intvector_75::Intvector75Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h79"]
pub mod intvector_75;
#[doc = "INTVECTOR_76 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h80\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_76`]
module"]
#[doc(alias = "INTVECTOR_76")]
pub type Intvector76 = crate::Reg<intvector_76::Intvector76Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h80"]
pub mod intvector_76;
#[doc = "INTVECTOR_77 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h81\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_77`]
module"]
#[doc(alias = "INTVECTOR_77")]
pub type Intvector77 = crate::Reg<intvector_77::Intvector77Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h81"]
pub mod intvector_77;
#[doc = "INTVECTOR_78 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h82\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_78`]
module"]
#[doc(alias = "INTVECTOR_78")]
pub type Intvector78 = crate::Reg<intvector_78::Intvector78Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h82"]
pub mod intvector_78;
#[doc = "INTVECTOR_79 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h83\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_79`]
module"]
#[doc(alias = "INTVECTOR_79")]
pub type Intvector79 = crate::Reg<intvector_79::Intvector79Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h83"]
pub mod intvector_79;
#[doc = "INTVECTOR_80 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h84\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_80`]
module"]
#[doc(alias = "INTVECTOR_80")]
pub type Intvector80 = crate::Reg<intvector_80::Intvector80Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h84"]
pub mod intvector_80;
#[doc = "INTVECTOR_81 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h85\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_81`]
module"]
#[doc(alias = "INTVECTOR_81")]
pub type Intvector81 = crate::Reg<intvector_81::Intvector81Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h85"]
pub mod intvector_81;
#[doc = "INTVECTOR_82 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h86\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_82`]
module"]
#[doc(alias = "INTVECTOR_82")]
pub type Intvector82 = crate::Reg<intvector_82::Intvector82Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h86"]
pub mod intvector_82;
#[doc = "INTVECTOR_83 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h87\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_83`]
module"]
#[doc(alias = "INTVECTOR_83")]
pub type Intvector83 = crate::Reg<intvector_83::Intvector83Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h87"]
pub mod intvector_83;
#[doc = "INTVECTOR_84 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h88\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_84`]
module"]
#[doc(alias = "INTVECTOR_84")]
pub type Intvector84 = crate::Reg<intvector_84::Intvector84Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h88"]
pub mod intvector_84;
#[doc = "INTVECTOR_85 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h89\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_85`]
module"]
#[doc(alias = "INTVECTOR_85")]
pub type Intvector85 = crate::Reg<intvector_85::Intvector85Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h89"]
pub mod intvector_85;
#[doc = "INTVECTOR_86 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h90\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_86`]
module"]
#[doc(alias = "INTVECTOR_86")]
pub type Intvector86 = crate::Reg<intvector_86::Intvector86Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h90"]
pub mod intvector_86;
#[doc = "INTVECTOR_87 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h91\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_87`]
module"]
#[doc(alias = "INTVECTOR_87")]
pub type Intvector87 = crate::Reg<intvector_87::Intvector87Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h91"]
pub mod intvector_87;
#[doc = "INTVECTOR_88 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h92\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_88`]
module"]
#[doc(alias = "INTVECTOR_88")]
pub type Intvector88 = crate::Reg<intvector_88::Intvector88Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h92"]
pub mod intvector_88;
#[doc = "INTVECTOR_89 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h93\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_89`]
module"]
#[doc(alias = "INTVECTOR_89")]
pub type Intvector89 = crate::Reg<intvector_89::Intvector89Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h93"]
pub mod intvector_89;
#[doc = "INTVECTOR_90 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h94\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_90`]
module"]
#[doc(alias = "INTVECTOR_90")]
pub type Intvector90 = crate::Reg<intvector_90::Intvector90Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h94"]
pub mod intvector_90;
#[doc = "INTVECTOR_91 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h95\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_91`]
module"]
#[doc(alias = "INTVECTOR_91")]
pub type Intvector91 = crate::Reg<intvector_91::Intvector91Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h95"]
pub mod intvector_91;
#[doc = "INTVECTOR_92 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h96\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_92`]
module"]
#[doc(alias = "INTVECTOR_92")]
pub type Intvector92 = crate::Reg<intvector_92::Intvector92Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h96"]
pub mod intvector_92;
#[doc = "INTVECTOR_93 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h97\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_93`]
module"]
#[doc(alias = "INTVECTOR_93")]
pub type Intvector93 = crate::Reg<intvector_93::Intvector93Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h97"]
pub mod intvector_93;
#[doc = "INTVECTOR_94 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h98\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_94`]
module"]
#[doc(alias = "INTVECTOR_94")]
pub type Intvector94 = crate::Reg<intvector_94::Intvector94Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h98"]
pub mod intvector_94;
#[doc = "INTVECTOR_95 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h99\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_95`]
module"]
#[doc(alias = "INTVECTOR_95")]
pub type Intvector95 = crate::Reg<intvector_95::Intvector95Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h99"]
pub mod intvector_95;
#[doc = "INTVECTOR_96 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h100\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_96`]
module"]
#[doc(alias = "INTVECTOR_96")]
pub type Intvector96 = crate::Reg<intvector_96::Intvector96Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h100"]
pub mod intvector_96;
#[doc = "INTVECTOR_97 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h101\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_97`]
module"]
#[doc(alias = "INTVECTOR_97")]
pub type Intvector97 = crate::Reg<intvector_97::Intvector97Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h101"]
pub mod intvector_97;
#[doc = "INTVECTOR_98 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h102\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_98`]
module"]
#[doc(alias = "INTVECTOR_98")]
pub type Intvector98 = crate::Reg<intvector_98::Intvector98Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h102"]
pub mod intvector_98;
#[doc = "INTVECTOR_99 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h103\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_99`]
module"]
#[doc(alias = "INTVECTOR_99")]
pub type Intvector99 = crate::Reg<intvector_99::Intvector99Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h103"]
pub mod intvector_99;
#[doc = "INTVECTOR_100 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h104\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_100`]
module"]
#[doc(alias = "INTVECTOR_100")]
pub type Intvector100 = crate::Reg<intvector_100::Intvector100Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h104"]
pub mod intvector_100;
#[doc = "INTVECTOR_101 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h105\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_101::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_101`]
module"]
#[doc(alias = "INTVECTOR_101")]
pub type Intvector101 = crate::Reg<intvector_101::Intvector101Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h105"]
pub mod intvector_101;
#[doc = "INTVECTOR_102 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h106\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_102::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_102`]
module"]
#[doc(alias = "INTVECTOR_102")]
pub type Intvector102 = crate::Reg<intvector_102::Intvector102Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h106"]
pub mod intvector_102;
#[doc = "INTVECTOR_103 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h107\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_103::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_103::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_103`]
module"]
#[doc(alias = "INTVECTOR_103")]
pub type Intvector103 = crate::Reg<intvector_103::Intvector103Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h107"]
pub mod intvector_103;
#[doc = "INTVECTOR_104 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h108\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_104`]
module"]
#[doc(alias = "INTVECTOR_104")]
pub type Intvector104 = crate::Reg<intvector_104::Intvector104Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h108"]
pub mod intvector_104;
#[doc = "INTVECTOR_105 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h109\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_105::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_105::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_105`]
module"]
#[doc(alias = "INTVECTOR_105")]
pub type Intvector105 = crate::Reg<intvector_105::Intvector105Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h109"]
pub mod intvector_105;
#[doc = "INTVECTOR_106 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h110\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_106::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_106::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_106`]
module"]
#[doc(alias = "INTVECTOR_106")]
pub type Intvector106 = crate::Reg<intvector_106::Intvector106Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h110"]
pub mod intvector_106;
#[doc = "INTVECTOR_107 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h111\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_107::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_107::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_107`]
module"]
#[doc(alias = "INTVECTOR_107")]
pub type Intvector107 = crate::Reg<intvector_107::Intvector107Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h111"]
pub mod intvector_107;
#[doc = "INTVECTOR_108 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h112\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_108`]
module"]
#[doc(alias = "INTVECTOR_108")]
pub type Intvector108 = crate::Reg<intvector_108::Intvector108Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h112"]
pub mod intvector_108;
#[doc = "INTVECTOR_109 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h113\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_109::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_109::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_109`]
module"]
#[doc(alias = "INTVECTOR_109")]
pub type Intvector109 = crate::Reg<intvector_109::Intvector109Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h113"]
pub mod intvector_109;
#[doc = "INTVECTOR_110 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h114\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_110`]
module"]
#[doc(alias = "INTVECTOR_110")]
pub type Intvector110 = crate::Reg<intvector_110::Intvector110Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h114"]
pub mod intvector_110;
#[doc = "INTVECTOR_111 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h115\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_111`]
module"]
#[doc(alias = "INTVECTOR_111")]
pub type Intvector111 = crate::Reg<intvector_111::Intvector111Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h115"]
pub mod intvector_111;
#[doc = "INTVECTOR_112 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h116\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_112`]
module"]
#[doc(alias = "INTVECTOR_112")]
pub type Intvector112 = crate::Reg<intvector_112::Intvector112Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h116"]
pub mod intvector_112;
#[doc = "INTVECTOR_113 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h117\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_113`]
module"]
#[doc(alias = "INTVECTOR_113")]
pub type Intvector113 = crate::Reg<intvector_113::Intvector113Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h117"]
pub mod intvector_113;
#[doc = "INTVECTOR_114 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h118\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_114`]
module"]
#[doc(alias = "INTVECTOR_114")]
pub type Intvector114 = crate::Reg<intvector_114::Intvector114Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h118"]
pub mod intvector_114;
#[doc = "INTVECTOR_115 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h119\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_115`]
module"]
#[doc(alias = "INTVECTOR_115")]
pub type Intvector115 = crate::Reg<intvector_115::Intvector115Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h119"]
pub mod intvector_115;
#[doc = "INTVECTOR_116 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h120\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_116`]
module"]
#[doc(alias = "INTVECTOR_116")]
pub type Intvector116 = crate::Reg<intvector_116::Intvector116Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h120"]
pub mod intvector_116;
#[doc = "INTVECTOR_117 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h121\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_117`]
module"]
#[doc(alias = "INTVECTOR_117")]
pub type Intvector117 = crate::Reg<intvector_117::Intvector117Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h121"]
pub mod intvector_117;
#[doc = "INTVECTOR_118 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h122\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_118`]
module"]
#[doc(alias = "INTVECTOR_118")]
pub type Intvector118 = crate::Reg<intvector_118::Intvector118Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h122"]
pub mod intvector_118;
#[doc = "INTVECTOR_119 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h123\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_119`]
module"]
#[doc(alias = "INTVECTOR_119")]
pub type Intvector119 = crate::Reg<intvector_119::Intvector119Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h123"]
pub mod intvector_119;
#[doc = "INTVECTOR_120 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h124\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_120`]
module"]
#[doc(alias = "INTVECTOR_120")]
pub type Intvector120 = crate::Reg<intvector_120::Intvector120Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h124"]
pub mod intvector_120;
#[doc = "INTVECTOR_121 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h125\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_121`]
module"]
#[doc(alias = "INTVECTOR_121")]
pub type Intvector121 = crate::Reg<intvector_121::Intvector121Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h125"]
pub mod intvector_121;
#[doc = "INTVECTOR_122 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h126\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_122`]
module"]
#[doc(alias = "INTVECTOR_122")]
pub type Intvector122 = crate::Reg<intvector_122::Intvector122Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h126"]
pub mod intvector_122;
#[doc = "INTVECTOR_123 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h127\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_123`]
module"]
#[doc(alias = "INTVECTOR_123")]
pub type Intvector123 = crate::Reg<intvector_123::Intvector123Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h127"]
pub mod intvector_123;
#[doc = "INTVECTOR_124 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h128\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_124`]
module"]
#[doc(alias = "INTVECTOR_124")]
pub type Intvector124 = crate::Reg<intvector_124::Intvector124Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h128"]
pub mod intvector_124;
#[doc = "INTVECTOR_125 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h129\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_125`]
module"]
#[doc(alias = "INTVECTOR_125")]
pub type Intvector125 = crate::Reg<intvector_125::Intvector125Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h129"]
pub mod intvector_125;
#[doc = "INTVECTOR_126 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h130\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_126`]
module"]
#[doc(alias = "INTVECTOR_126")]
pub type Intvector126 = crate::Reg<intvector_126::Intvector126Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h130"]
pub mod intvector_126;
#[doc = "INTVECTOR_127 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h131\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_127`]
module"]
#[doc(alias = "INTVECTOR_127")]
pub type Intvector127 = crate::Reg<intvector_127::Intvector127Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h131"]
pub mod intvector_127;
#[doc = "INTVECTOR_128 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h132\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_128`]
module"]
#[doc(alias = "INTVECTOR_128")]
pub type Intvector128 = crate::Reg<intvector_128::Intvector128Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h132"]
pub mod intvector_128;
#[doc = "INTVECTOR_129 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h133\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_129::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_129::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_129`]
module"]
#[doc(alias = "INTVECTOR_129")]
pub type Intvector129 = crate::Reg<intvector_129::Intvector129Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h133"]
pub mod intvector_129;
#[doc = "INTVECTOR_130 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h134\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_130`]
module"]
#[doc(alias = "INTVECTOR_130")]
pub type Intvector130 = crate::Reg<intvector_130::Intvector130Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h134"]
pub mod intvector_130;
#[doc = "INTVECTOR_131 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h135\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_131::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_131::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_131`]
module"]
#[doc(alias = "INTVECTOR_131")]
pub type Intvector131 = crate::Reg<intvector_131::Intvector131Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h135"]
pub mod intvector_131;
#[doc = "INTVECTOR_132 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h136\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_132::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_132::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_132`]
module"]
#[doc(alias = "INTVECTOR_132")]
pub type Intvector132 = crate::Reg<intvector_132::Intvector132Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h136"]
pub mod intvector_132;
#[doc = "INTVECTOR_133 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h137\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_133::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_133::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_133`]
module"]
#[doc(alias = "INTVECTOR_133")]
pub type Intvector133 = crate::Reg<intvector_133::Intvector133Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h137"]
pub mod intvector_133;
#[doc = "INTVECTOR_134 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h138\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_134`]
module"]
#[doc(alias = "INTVECTOR_134")]
pub type Intvector134 = crate::Reg<intvector_134::Intvector134Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h138"]
pub mod intvector_134;
#[doc = "INTVECTOR_135 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h139\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_135::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_135::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_135`]
module"]
#[doc(alias = "INTVECTOR_135")]
pub type Intvector135 = crate::Reg<intvector_135::Intvector135Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h139"]
pub mod intvector_135;
#[doc = "INTVECTOR_136 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h140\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_136::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_136::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_136`]
module"]
#[doc(alias = "INTVECTOR_136")]
pub type Intvector136 = crate::Reg<intvector_136::Intvector136Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h140"]
pub mod intvector_136;
#[doc = "INTVECTOR_137 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h141\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_137::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_137::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_137`]
module"]
#[doc(alias = "INTVECTOR_137")]
pub type Intvector137 = crate::Reg<intvector_137::Intvector137Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h141"]
pub mod intvector_137;
#[doc = "INTVECTOR_138 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h142\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_138`]
module"]
#[doc(alias = "INTVECTOR_138")]
pub type Intvector138 = crate::Reg<intvector_138::Intvector138Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h142"]
pub mod intvector_138;
#[doc = "INTVECTOR_139 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h143\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_139::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_139::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_139`]
module"]
#[doc(alias = "INTVECTOR_139")]
pub type Intvector139 = crate::Reg<intvector_139::Intvector139Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h143"]
pub mod intvector_139;
#[doc = "INTVECTOR_140 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h144\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_140`]
module"]
#[doc(alias = "INTVECTOR_140")]
pub type Intvector140 = crate::Reg<intvector_140::Intvector140Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h144"]
pub mod intvector_140;
#[doc = "INTVECTOR_141 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h145\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_141::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_141::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_141`]
module"]
#[doc(alias = "INTVECTOR_141")]
pub type Intvector141 = crate::Reg<intvector_141::Intvector141Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h145"]
pub mod intvector_141;
#[doc = "INTVECTOR_142 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h146\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_142::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_142::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_142`]
module"]
#[doc(alias = "INTVECTOR_142")]
pub type Intvector142 = crate::Reg<intvector_142::Intvector142Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h146"]
pub mod intvector_142;
#[doc = "INTVECTOR_143 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h147\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_143::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_143::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_143`]
module"]
#[doc(alias = "INTVECTOR_143")]
pub type Intvector143 = crate::Reg<intvector_143::Intvector143Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h147"]
pub mod intvector_143;
#[doc = "INTVECTOR_144 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h148\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_144`]
module"]
#[doc(alias = "INTVECTOR_144")]
pub type Intvector144 = crate::Reg<intvector_144::Intvector144Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h148"]
pub mod intvector_144;
#[doc = "INTVECTOR_145 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h149\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_145::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_145::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_145`]
module"]
#[doc(alias = "INTVECTOR_145")]
pub type Intvector145 = crate::Reg<intvector_145::Intvector145Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h149"]
pub mod intvector_145;
#[doc = "INTVECTOR_146 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h150\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_146::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_146::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_146`]
module"]
#[doc(alias = "INTVECTOR_146")]
pub type Intvector146 = crate::Reg<intvector_146::Intvector146Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h150"]
pub mod intvector_146;
#[doc = "INTVECTOR_147 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h151\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_147::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_147::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_147`]
module"]
#[doc(alias = "INTVECTOR_147")]
pub type Intvector147 = crate::Reg<intvector_147::Intvector147Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h151"]
pub mod intvector_147;
#[doc = "INTVECTOR_148 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h152\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_148`]
module"]
#[doc(alias = "INTVECTOR_148")]
pub type Intvector148 = crate::Reg<intvector_148::Intvector148Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h152"]
pub mod intvector_148;
#[doc = "INTVECTOR_149 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h153\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_149::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_149::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_149`]
module"]
#[doc(alias = "INTVECTOR_149")]
pub type Intvector149 = crate::Reg<intvector_149::Intvector149Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h153"]
pub mod intvector_149;
#[doc = "INTVECTOR_150 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h154\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_150`]
module"]
#[doc(alias = "INTVECTOR_150")]
pub type Intvector150 = crate::Reg<intvector_150::Intvector150Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h154"]
pub mod intvector_150;
#[doc = "INTVECTOR_151 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h155\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_151::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_151::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_151`]
module"]
#[doc(alias = "INTVECTOR_151")]
pub type Intvector151 = crate::Reg<intvector_151::Intvector151Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h155"]
pub mod intvector_151;
#[doc = "INTVECTOR_152 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h156\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_152::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_152::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_152`]
module"]
#[doc(alias = "INTVECTOR_152")]
pub type Intvector152 = crate::Reg<intvector_152::Intvector152Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h156"]
pub mod intvector_152;
#[doc = "INTVECTOR_153 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h157\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_153::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_153::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_153`]
module"]
#[doc(alias = "INTVECTOR_153")]
pub type Intvector153 = crate::Reg<intvector_153::Intvector153Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h157"]
pub mod intvector_153;
#[doc = "INTVECTOR_154 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h158\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_154`]
module"]
#[doc(alias = "INTVECTOR_154")]
pub type Intvector154 = crate::Reg<intvector_154::Intvector154Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h158"]
pub mod intvector_154;
#[doc = "INTVECTOR_155 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h159\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_155::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_155::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_155`]
module"]
#[doc(alias = "INTVECTOR_155")]
pub type Intvector155 = crate::Reg<intvector_155::Intvector155Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h159"]
pub mod intvector_155;
#[doc = "INTVECTOR_156 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h160\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_156::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_156::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_156`]
module"]
#[doc(alias = "INTVECTOR_156")]
pub type Intvector156 = crate::Reg<intvector_156::Intvector156Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h160"]
pub mod intvector_156;
#[doc = "INTVECTOR_157 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h161\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_157::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_157::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_157`]
module"]
#[doc(alias = "INTVECTOR_157")]
pub type Intvector157 = crate::Reg<intvector_157::Intvector157Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h161"]
pub mod intvector_157;
#[doc = "INTVECTOR_158 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h162\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_158`]
module"]
#[doc(alias = "INTVECTOR_158")]
pub type Intvector158 = crate::Reg<intvector_158::Intvector158Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h162"]
pub mod intvector_158;
#[doc = "INTVECTOR_159 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h163\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_159::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_159::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_159`]
module"]
#[doc(alias = "INTVECTOR_159")]
pub type Intvector159 = crate::Reg<intvector_159::Intvector159Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h163"]
pub mod intvector_159;
#[doc = "INTVECTOR_160 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h164\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_160`]
module"]
#[doc(alias = "INTVECTOR_160")]
pub type Intvector160 = crate::Reg<intvector_160::Intvector160Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h164"]
pub mod intvector_160;
#[doc = "INTVECTOR_161 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h165\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_161::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_161::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_161`]
module"]
#[doc(alias = "INTVECTOR_161")]
pub type Intvector161 = crate::Reg<intvector_161::Intvector161Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h165"]
pub mod intvector_161;
#[doc = "INTVECTOR_162 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h166\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_162::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_162::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_162`]
module"]
#[doc(alias = "INTVECTOR_162")]
pub type Intvector162 = crate::Reg<intvector_162::Intvector162Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h166"]
pub mod intvector_162;
#[doc = "INTVECTOR_163 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h167\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_163::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_163::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_163`]
module"]
#[doc(alias = "INTVECTOR_163")]
pub type Intvector163 = crate::Reg<intvector_163::Intvector163Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h167"]
pub mod intvector_163;
#[doc = "INTVECTOR_164 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h168\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_164`]
module"]
#[doc(alias = "INTVECTOR_164")]
pub type Intvector164 = crate::Reg<intvector_164::Intvector164Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h168"]
pub mod intvector_164;
#[doc = "INTVECTOR_165 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h169\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_165::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_165::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_165`]
module"]
#[doc(alias = "INTVECTOR_165")]
pub type Intvector165 = crate::Reg<intvector_165::Intvector165Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h169"]
pub mod intvector_165;
#[doc = "INTVECTOR_166 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h170\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_166::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_166::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_166`]
module"]
#[doc(alias = "INTVECTOR_166")]
pub type Intvector166 = crate::Reg<intvector_166::Intvector166Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h170"]
pub mod intvector_166;
#[doc = "INTVECTOR_167 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h171\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_167::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_167::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_167`]
module"]
#[doc(alias = "INTVECTOR_167")]
pub type Intvector167 = crate::Reg<intvector_167::Intvector167Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h171"]
pub mod intvector_167;
#[doc = "INTVECTOR_168 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h172\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_168`]
module"]
#[doc(alias = "INTVECTOR_168")]
pub type Intvector168 = crate::Reg<intvector_168::Intvector168Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h172"]
pub mod intvector_168;
#[doc = "INTVECTOR_169 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h173\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_169::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_169::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_169`]
module"]
#[doc(alias = "INTVECTOR_169")]
pub type Intvector169 = crate::Reg<intvector_169::Intvector169Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h173"]
pub mod intvector_169;
#[doc = "INTVECTOR_170 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h174\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_170`]
module"]
#[doc(alias = "INTVECTOR_170")]
pub type Intvector170 = crate::Reg<intvector_170::Intvector170Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h174"]
pub mod intvector_170;
#[doc = "INTVECTOR_171 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h175\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_171::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_171::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_171`]
module"]
#[doc(alias = "INTVECTOR_171")]
pub type Intvector171 = crate::Reg<intvector_171::Intvector171Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h175"]
pub mod intvector_171;
#[doc = "INTVECTOR_172 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h176\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_172::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_172::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_172`]
module"]
#[doc(alias = "INTVECTOR_172")]
pub type Intvector172 = crate::Reg<intvector_172::Intvector172Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h176"]
pub mod intvector_172;
#[doc = "INTVECTOR_173 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h177\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_173::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_173::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_173`]
module"]
#[doc(alias = "INTVECTOR_173")]
pub type Intvector173 = crate::Reg<intvector_173::Intvector173Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h177"]
pub mod intvector_173;
#[doc = "INTVECTOR_174 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h178\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_174`]
module"]
#[doc(alias = "INTVECTOR_174")]
pub type Intvector174 = crate::Reg<intvector_174::Intvector174Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h178"]
pub mod intvector_174;
#[doc = "INTVECTOR_175 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h179\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_175::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_175::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_175`]
module"]
#[doc(alias = "INTVECTOR_175")]
pub type Intvector175 = crate::Reg<intvector_175::Intvector175Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h179"]
pub mod intvector_175;
#[doc = "INTVECTOR_176 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h180\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_176::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_176::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_176`]
module"]
#[doc(alias = "INTVECTOR_176")]
pub type Intvector176 = crate::Reg<intvector_176::Intvector176Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h180"]
pub mod intvector_176;
#[doc = "INTVECTOR_177 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h181\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_177::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_177::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_177`]
module"]
#[doc(alias = "INTVECTOR_177")]
pub type Intvector177 = crate::Reg<intvector_177::Intvector177Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h181"]
pub mod intvector_177;
#[doc = "INTVECTOR_178 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h182\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_178`]
module"]
#[doc(alias = "INTVECTOR_178")]
pub type Intvector178 = crate::Reg<intvector_178::Intvector178Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h182"]
pub mod intvector_178;
#[doc = "INTVECTOR_179 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h183\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_179::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_179::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_179`]
module"]
#[doc(alias = "INTVECTOR_179")]
pub type Intvector179 = crate::Reg<intvector_179::Intvector179Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h183"]
pub mod intvector_179;
#[doc = "INTVECTOR_180 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h184\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_180`]
module"]
#[doc(alias = "INTVECTOR_180")]
pub type Intvector180 = crate::Reg<intvector_180::Intvector180Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h184"]
pub mod intvector_180;
#[doc = "INTVECTOR_181 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h185\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_181::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_181::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_181`]
module"]
#[doc(alias = "INTVECTOR_181")]
pub type Intvector181 = crate::Reg<intvector_181::Intvector181Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h185"]
pub mod intvector_181;
#[doc = "INTVECTOR_182 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h186\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_182::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_182::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_182`]
module"]
#[doc(alias = "INTVECTOR_182")]
pub type Intvector182 = crate::Reg<intvector_182::Intvector182Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h186"]
pub mod intvector_182;
#[doc = "INTVECTOR_183 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h187\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_183::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_183::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_183`]
module"]
#[doc(alias = "INTVECTOR_183")]
pub type Intvector183 = crate::Reg<intvector_183::Intvector183Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h187"]
pub mod intvector_183;
#[doc = "INTVECTOR_184 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h188\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_184`]
module"]
#[doc(alias = "INTVECTOR_184")]
pub type Intvector184 = crate::Reg<intvector_184::Intvector184Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h188"]
pub mod intvector_184;
#[doc = "INTVECTOR_185 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h189\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_185::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_185::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_185`]
module"]
#[doc(alias = "INTVECTOR_185")]
pub type Intvector185 = crate::Reg<intvector_185::Intvector185Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h189"]
pub mod intvector_185;
#[doc = "INTVECTOR_186 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h190\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_186::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_186::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_186`]
module"]
#[doc(alias = "INTVECTOR_186")]
pub type Intvector186 = crate::Reg<intvector_186::Intvector186Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h190"]
pub mod intvector_186;
#[doc = "INTVECTOR_187 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h191\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_187::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_187::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_187`]
module"]
#[doc(alias = "INTVECTOR_187")]
pub type Intvector187 = crate::Reg<intvector_187::Intvector187Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h191"]
pub mod intvector_187;
#[doc = "INTVECTOR_188 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h192\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_188`]
module"]
#[doc(alias = "INTVECTOR_188")]
pub type Intvector188 = crate::Reg<intvector_188::Intvector188Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h192"]
pub mod intvector_188;
#[doc = "INTVECTOR_189 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h193\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_189::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_189::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_189`]
module"]
#[doc(alias = "INTVECTOR_189")]
pub type Intvector189 = crate::Reg<intvector_189::Intvector189Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h193"]
pub mod intvector_189;
#[doc = "INTVECTOR_190 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h194\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_190`]
module"]
#[doc(alias = "INTVECTOR_190")]
pub type Intvector190 = crate::Reg<intvector_190::Intvector190Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h194"]
pub mod intvector_190;
#[doc = "INTVECTOR_191 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h195\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_191::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_191::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_191`]
module"]
#[doc(alias = "INTVECTOR_191")]
pub type Intvector191 = crate::Reg<intvector_191::Intvector191Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h195"]
pub mod intvector_191;
#[doc = "INTVECTOR_192 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h196\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_192::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_192::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_192`]
module"]
#[doc(alias = "INTVECTOR_192")]
pub type Intvector192 = crate::Reg<intvector_192::Intvector192Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h196"]
pub mod intvector_192;
#[doc = "INTVECTOR_193 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h197\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_193::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_193::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_193`]
module"]
#[doc(alias = "INTVECTOR_193")]
pub type Intvector193 = crate::Reg<intvector_193::Intvector193Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h197"]
pub mod intvector_193;
#[doc = "INTVECTOR_194 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h198\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_194`]
module"]
#[doc(alias = "INTVECTOR_194")]
pub type Intvector194 = crate::Reg<intvector_194::Intvector194Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h198"]
pub mod intvector_194;
#[doc = "INTVECTOR_195 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h199\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_195::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_195::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_195`]
module"]
#[doc(alias = "INTVECTOR_195")]
pub type Intvector195 = crate::Reg<intvector_195::Intvector195Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h199"]
pub mod intvector_195;
#[doc = "INTVECTOR_196 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h200\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_196::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_196::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_196`]
module"]
#[doc(alias = "INTVECTOR_196")]
pub type Intvector196 = crate::Reg<intvector_196::Intvector196Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h200"]
pub mod intvector_196;
#[doc = "INTVECTOR_197 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h201\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_197::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_197::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_197`]
module"]
#[doc(alias = "INTVECTOR_197")]
pub type Intvector197 = crate::Reg<intvector_197::Intvector197Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h201"]
pub mod intvector_197;
#[doc = "INTVECTOR_198 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h202\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_198`]
module"]
#[doc(alias = "INTVECTOR_198")]
pub type Intvector198 = crate::Reg<intvector_198::Intvector198Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h202"]
pub mod intvector_198;
#[doc = "INTVECTOR_199 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h203\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_199::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_199::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_199`]
module"]
#[doc(alias = "INTVECTOR_199")]
pub type Intvector199 = crate::Reg<intvector_199::Intvector199Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h203"]
pub mod intvector_199;
#[doc = "INTVECTOR_200 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h204\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_200`]
module"]
#[doc(alias = "INTVECTOR_200")]
pub type Intvector200 = crate::Reg<intvector_200::Intvector200Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h204"]
pub mod intvector_200;
#[doc = "INTVECTOR_201 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h205\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_201::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_201::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_201`]
module"]
#[doc(alias = "INTVECTOR_201")]
pub type Intvector201 = crate::Reg<intvector_201::Intvector201Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h205"]
pub mod intvector_201;
#[doc = "INTVECTOR_202 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h206\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_202::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_202::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_202`]
module"]
#[doc(alias = "INTVECTOR_202")]
pub type Intvector202 = crate::Reg<intvector_202::Intvector202Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h206"]
pub mod intvector_202;
#[doc = "INTVECTOR_203 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h207\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_203::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_203::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_203`]
module"]
#[doc(alias = "INTVECTOR_203")]
pub type Intvector203 = crate::Reg<intvector_203::Intvector203Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h207"]
pub mod intvector_203;
#[doc = "INTVECTOR_204 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h208\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_204`]
module"]
#[doc(alias = "INTVECTOR_204")]
pub type Intvector204 = crate::Reg<intvector_204::Intvector204Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h208"]
pub mod intvector_204;
#[doc = "INTVECTOR_205 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h209\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_205::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_205::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_205`]
module"]
#[doc(alias = "INTVECTOR_205")]
pub type Intvector205 = crate::Reg<intvector_205::Intvector205Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h209"]
pub mod intvector_205;
#[doc = "INTVECTOR_206 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h210\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_206::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_206::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_206`]
module"]
#[doc(alias = "INTVECTOR_206")]
pub type Intvector206 = crate::Reg<intvector_206::Intvector206Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h210"]
pub mod intvector_206;
#[doc = "INTVECTOR_207 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h211\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_207::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_207::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_207`]
module"]
#[doc(alias = "INTVECTOR_207")]
pub type Intvector207 = crate::Reg<intvector_207::Intvector207Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h211"]
pub mod intvector_207;
#[doc = "INTVECTOR_208 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h212\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_208`]
module"]
#[doc(alias = "INTVECTOR_208")]
pub type Intvector208 = crate::Reg<intvector_208::Intvector208Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h212"]
pub mod intvector_208;
#[doc = "INTVECTOR_209 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h213\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_209::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_209::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_209`]
module"]
#[doc(alias = "INTVECTOR_209")]
pub type Intvector209 = crate::Reg<intvector_209::Intvector209Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h213"]
pub mod intvector_209;
#[doc = "INTVECTOR_210 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h214\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_210`]
module"]
#[doc(alias = "INTVECTOR_210")]
pub type Intvector210 = crate::Reg<intvector_210::Intvector210Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h214"]
pub mod intvector_210;
#[doc = "INTVECTOR_211 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h215\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_211::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_211::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_211`]
module"]
#[doc(alias = "INTVECTOR_211")]
pub type Intvector211 = crate::Reg<intvector_211::Intvector211Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h215"]
pub mod intvector_211;
#[doc = "INTVECTOR_212 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h216\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_212::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_212::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_212`]
module"]
#[doc(alias = "INTVECTOR_212")]
pub type Intvector212 = crate::Reg<intvector_212::Intvector212Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h216"]
pub mod intvector_212;
#[doc = "INTVECTOR_213 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h217\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_213::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_213::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_213`]
module"]
#[doc(alias = "INTVECTOR_213")]
pub type Intvector213 = crate::Reg<intvector_213::Intvector213Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h217"]
pub mod intvector_213;
#[doc = "INTVECTOR_214 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h218\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_214`]
module"]
#[doc(alias = "INTVECTOR_214")]
pub type Intvector214 = crate::Reg<intvector_214::Intvector214Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h218"]
pub mod intvector_214;
#[doc = "INTVECTOR_215 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h219\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_215::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_215::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_215`]
module"]
#[doc(alias = "INTVECTOR_215")]
pub type Intvector215 = crate::Reg<intvector_215::Intvector215Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h219"]
pub mod intvector_215;
#[doc = "INTVECTOR_216 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h220\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_216::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_216::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_216`]
module"]
#[doc(alias = "INTVECTOR_216")]
pub type Intvector216 = crate::Reg<intvector_216::Intvector216Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h220"]
pub mod intvector_216;
#[doc = "INTVECTOR_217 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h221\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_217::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_217::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_217`]
module"]
#[doc(alias = "INTVECTOR_217")]
pub type Intvector217 = crate::Reg<intvector_217::Intvector217Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h221"]
pub mod intvector_217;
#[doc = "INTVECTOR_218 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h222\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_218`]
module"]
#[doc(alias = "INTVECTOR_218")]
pub type Intvector218 = crate::Reg<intvector_218::Intvector218Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h222"]
pub mod intvector_218;
#[doc = "INTVECTOR_219 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h223\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_219::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_219::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_219`]
module"]
#[doc(alias = "INTVECTOR_219")]
pub type Intvector219 = crate::Reg<intvector_219::Intvector219Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h223"]
pub mod intvector_219;
#[doc = "INTVECTOR_220 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h224\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_220`]
module"]
#[doc(alias = "INTVECTOR_220")]
pub type Intvector220 = crate::Reg<intvector_220::Intvector220Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h224"]
pub mod intvector_220;
#[doc = "INTVECTOR_221 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h225\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_221::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_221::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_221`]
module"]
#[doc(alias = "INTVECTOR_221")]
pub type Intvector221 = crate::Reg<intvector_221::Intvector221Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h225"]
pub mod intvector_221;
#[doc = "INTVECTOR_222 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h226\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_222::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_222::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_222`]
module"]
#[doc(alias = "INTVECTOR_222")]
pub type Intvector222 = crate::Reg<intvector_222::Intvector222Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h226"]
pub mod intvector_222;
#[doc = "INTVECTOR_223 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h227\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_223::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_223::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_223`]
module"]
#[doc(alias = "INTVECTOR_223")]
pub type Intvector223 = crate::Reg<intvector_223::Intvector223Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h227"]
pub mod intvector_223;
#[doc = "INTVECTOR_224 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h228\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_224`]
module"]
#[doc(alias = "INTVECTOR_224")]
pub type Intvector224 = crate::Reg<intvector_224::Intvector224Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h228"]
pub mod intvector_224;
#[doc = "INTVECTOR_225 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h229\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_225::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_225::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_225`]
module"]
#[doc(alias = "INTVECTOR_225")]
pub type Intvector225 = crate::Reg<intvector_225::Intvector225Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h229"]
pub mod intvector_225;
#[doc = "INTVECTOR_226 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h230\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_226::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_226::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_226`]
module"]
#[doc(alias = "INTVECTOR_226")]
pub type Intvector226 = crate::Reg<intvector_226::Intvector226Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h230"]
pub mod intvector_226;
#[doc = "INTVECTOR_227 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h231\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_227::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_227::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_227`]
module"]
#[doc(alias = "INTVECTOR_227")]
pub type Intvector227 = crate::Reg<intvector_227::Intvector227Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h231"]
pub mod intvector_227;
#[doc = "INTVECTOR_228 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h232\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_228`]
module"]
#[doc(alias = "INTVECTOR_228")]
pub type Intvector228 = crate::Reg<intvector_228::Intvector228Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h232"]
pub mod intvector_228;
#[doc = "INTVECTOR_229 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h233\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_229::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_229::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_229`]
module"]
#[doc(alias = "INTVECTOR_229")]
pub type Intvector229 = crate::Reg<intvector_229::Intvector229Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h233"]
pub mod intvector_229;
#[doc = "INTVECTOR_230 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h234\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_230`]
module"]
#[doc(alias = "INTVECTOR_230")]
pub type Intvector230 = crate::Reg<intvector_230::Intvector230Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h234"]
pub mod intvector_230;
#[doc = "INTVECTOR_231 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h235\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_231::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_231::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_231`]
module"]
#[doc(alias = "INTVECTOR_231")]
pub type Intvector231 = crate::Reg<intvector_231::Intvector231Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h235"]
pub mod intvector_231;
#[doc = "INTVECTOR_232 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h236\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_232::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_232::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_232`]
module"]
#[doc(alias = "INTVECTOR_232")]
pub type Intvector232 = crate::Reg<intvector_232::Intvector232Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h236"]
pub mod intvector_232;
#[doc = "INTVECTOR_233 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h237\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_233::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_233::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_233`]
module"]
#[doc(alias = "INTVECTOR_233")]
pub type Intvector233 = crate::Reg<intvector_233::Intvector233Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h237"]
pub mod intvector_233;
#[doc = "INTVECTOR_234 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h238\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_234`]
module"]
#[doc(alias = "INTVECTOR_234")]
pub type Intvector234 = crate::Reg<intvector_234::Intvector234Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h238"]
pub mod intvector_234;
#[doc = "INTVECTOR_235 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h239\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_235::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_235::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_235`]
module"]
#[doc(alias = "INTVECTOR_235")]
pub type Intvector235 = crate::Reg<intvector_235::Intvector235Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h239"]
pub mod intvector_235;
#[doc = "INTVECTOR_236 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h240\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_236::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_236::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_236`]
module"]
#[doc(alias = "INTVECTOR_236")]
pub type Intvector236 = crate::Reg<intvector_236::Intvector236Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h240"]
pub mod intvector_236;
#[doc = "INTVECTOR_237 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h241\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_237::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_237::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_237`]
module"]
#[doc(alias = "INTVECTOR_237")]
pub type Intvector237 = crate::Reg<intvector_237::Intvector237Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h241"]
pub mod intvector_237;
#[doc = "INTVECTOR_238 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h242\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_238`]
module"]
#[doc(alias = "INTVECTOR_238")]
pub type Intvector238 = crate::Reg<intvector_238::Intvector238Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h242"]
pub mod intvector_238;
#[doc = "INTVECTOR_239 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h243\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_239::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_239::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_239`]
module"]
#[doc(alias = "INTVECTOR_239")]
pub type Intvector239 = crate::Reg<intvector_239::Intvector239Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h243"]
pub mod intvector_239;
#[doc = "INTVECTOR_240 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h244\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_240`]
module"]
#[doc(alias = "INTVECTOR_240")]
pub type Intvector240 = crate::Reg<intvector_240::Intvector240Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h244"]
pub mod intvector_240;
#[doc = "INTVECTOR_241 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h245\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_241::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_241::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_241`]
module"]
#[doc(alias = "INTVECTOR_241")]
pub type Intvector241 = crate::Reg<intvector_241::Intvector241Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h245"]
pub mod intvector_241;
#[doc = "INTVECTOR_242 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h246\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_242::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_242::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_242`]
module"]
#[doc(alias = "INTVECTOR_242")]
pub type Intvector242 = crate::Reg<intvector_242::Intvector242Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h246"]
pub mod intvector_242;
#[doc = "INTVECTOR_243 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h247\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_243::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_243::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_243`]
module"]
#[doc(alias = "INTVECTOR_243")]
pub type Intvector243 = crate::Reg<intvector_243::Intvector243Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h247"]
pub mod intvector_243;
#[doc = "INTVECTOR_244 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h248\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_244::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_244`]
module"]
#[doc(alias = "INTVECTOR_244")]
pub type Intvector244 = crate::Reg<intvector_244::Intvector244Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h248"]
pub mod intvector_244;
#[doc = "INTVECTOR_245 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h249\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_245::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_245::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_245`]
module"]
#[doc(alias = "INTVECTOR_245")]
pub type Intvector245 = crate::Reg<intvector_245::Intvector245Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h249"]
pub mod intvector_245;
#[doc = "INTVECTOR_246 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h250\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_246::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_246::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_246`]
module"]
#[doc(alias = "INTVECTOR_246")]
pub type Intvector246 = crate::Reg<intvector_246::Intvector246Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h250"]
pub mod intvector_246;
#[doc = "INTVECTOR_247 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h251\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_247::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_247::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_247`]
module"]
#[doc(alias = "INTVECTOR_247")]
pub type Intvector247 = crate::Reg<intvector_247::Intvector247Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h251"]
pub mod intvector_247;
#[doc = "INTVECTOR_248 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h252\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_248::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_248`]
module"]
#[doc(alias = "INTVECTOR_248")]
pub type Intvector248 = crate::Reg<intvector_248::Intvector248Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h252"]
pub mod intvector_248;
#[doc = "INTVECTOR_249 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h253\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_249::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_249::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_249`]
module"]
#[doc(alias = "INTVECTOR_249")]
pub type Intvector249 = crate::Reg<intvector_249::Intvector249Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h253"]
pub mod intvector_249;
#[doc = "INTVECTOR_250 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h254\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_250`]
module"]
#[doc(alias = "INTVECTOR_250")]
pub type Intvector250 = crate::Reg<intvector_250::Intvector250Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h254"]
pub mod intvector_250;
#[doc = "INTVECTOR_251 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h255\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_251::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_251::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_251`]
module"]
#[doc(alias = "INTVECTOR_251")]
pub type Intvector251 = crate::Reg<intvector_251::Intvector251Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h255"]
pub mod intvector_251;
#[doc = "INTVECTOR_252 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h256\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_252::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_252::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_252`]
module"]
#[doc(alias = "INTVECTOR_252")]
pub type Intvector252 = crate::Reg<intvector_252::Intvector252Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h256"]
pub mod intvector_252;
#[doc = "INTVECTOR_253 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h257\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_253::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_253::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_253`]
module"]
#[doc(alias = "INTVECTOR_253")]
pub type Intvector253 = crate::Reg<intvector_253::Intvector253Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h257"]
pub mod intvector_253;
#[doc = "INTVECTOR_254 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h258\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_254::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_254`]
module"]
#[doc(alias = "INTVECTOR_254")]
pub type Intvector254 = crate::Reg<intvector_254::Intvector254Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h258"]
pub mod intvector_254;
#[doc = "INTVECTOR_255 (rw) register accessor: Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h259\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_255::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_255::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvector_255`]
module"]
#[doc(alias = "INTVECTOR_255")]
pub type Intvector255 = crate::Reg<intvector_255::Intvector255Spec>;
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h259"]
pub mod intvector_255;
