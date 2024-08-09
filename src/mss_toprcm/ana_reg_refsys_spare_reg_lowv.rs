#[doc = "Register `ANA_REG_REFSYS_SPARE_REG_LOWV` reader"]
pub type R = crate::R<AnaRegRefsysSpareRegLowvSpec>;
#[doc = "Register `ANA_REG_REFSYS_SPARE_REG_LOWV` writer"]
pub type W = crate::W<AnaRegRefsysSpareRegLowvSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Reserved In TPR, this bit is mapped to efuse and is used to control POR_DIG_SEQ_ECO_DIS. Writes have no effect. 0: Enable Slicer delay ECO 1: Disable Slicer delay ECO 0x0 = Functional Reset"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Reserved In TPR, this bit is mapped to efuse and is used to control POR_DIG_SEQ_ECO_DIS. Writes have no effect. 0: Enable Slicer delay ECO 1: Disable Slicer delay ECO 0x0 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 5:1\\]
Reserved Reads return 0x0 and writes have no effect, but these bits are tied to efuse overrides. 0x0 = Functional Reset"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 5:1\\]
Reserved Reads return 0x0 and writes have no effect, but these bits are tied to efuse overrides. 0x0 = Functional Reset"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VDD_SR_SEL` reader - 7:6\\]
Final level of VDD 1.2V VMON UV Reference Selection See definition in MSS_REFSYS_SPARE_REG&lt;23:22> for normal operation (MSS_REFSYS_SPARE_REG&lt;8> = 0x0) See definition in MSS_REFSYS_SPARE_REG&lt;15:14> for self-test operation (MSS_REFSYS_SPARE_REG&lt;8> = 0x1) 0x0 = Functional Reset"]
pub type VddSrSelR = crate::FieldReader;
#[doc = "Field `VDD_SR_SEL` writer - 7:6\\]
Final level of VDD 1.2V VMON UV Reference Selection See definition in MSS_REFSYS_SPARE_REG&lt;23:22> for normal operation (MSS_REFSYS_SPARE_REG&lt;8> = 0x0) See definition in MSS_REFSYS_SPARE_REG&lt;15:14> for self-test operation (MSS_REFSYS_SPARE_REG&lt;8> = 0x1) 0x0 = Functional Reset"]
pub type VddSrSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VDD_UV_SELF_TEST_SEL` reader - 8:8\\]
Enable 1.2V VDD Strict UV VMON Self Test If Self-test mode is enabled, VDD 1.2V VMON UV reference is programmed based on MSS_REFSYS_SPARE_REG&lt;15:14> and MSS_REFSYS_SPARE_REG&lt;7:6> as follows: If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.68V 0x1 = 0.67V 0x2 = 0.66V 0x3 = 0.65V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.65V 0x1 = 0.64V 0x2 = 0.63V 0x3 = 0.62V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.62V 0x1 = 0.61V 0x2 = 0.6V 0x3 = 0.59V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.59V 0x1 = 0.58V 0x2 = 0.57V 0x3 = 0.56V 0x0 = Functional Reset"]
pub type VddUvSelfTestSelR = crate::BitReader;
#[doc = "Field `VDD_UV_SELF_TEST_SEL` writer - 8:8\\]
Enable 1.2V VDD Strict UV VMON Self Test If Self-test mode is enabled, VDD 1.2V VMON UV reference is programmed based on MSS_REFSYS_SPARE_REG&lt;15:14> and MSS_REFSYS_SPARE_REG&lt;7:6> as follows: If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.68V 0x1 = 0.67V 0x2 = 0.66V 0x3 = 0.65V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.65V 0x1 = 0.64V 0x2 = 0.63V 0x3 = 0.62V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.62V 0x1 = 0.61V 0x2 = 0.6V 0x3 = 0.59V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.59V 0x1 = 0.58V 0x2 = 0.57V 0x3 = 0.56V 0x0 = Functional Reset"]
pub type VddUvSelfTestSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_OV_SELF_TEST_SEL` reader - 9:9\\]
Enable 1.2V VDD Strict OV VMON Self Test If Self-test mode is enabled, VDD 1.2V VMON OV reference is programmed based on MSS_REFSYS_SPARE_REG&lt;23:22> and MSS_REFSYS_SPARE_REG&lt;17:16> as follows: If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.58V 0x1 = 0.57V 0x2 = 0.56V 0x3 = 0.55V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.55V 0x1 = 0.54V 0x2 = 0.53V 0x3 = 0.52V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.53V 0x1 = 0.52V 0x2 = 0.51V 0x3 = 0.5V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.51V 0x1 = 0.5V 0x2 = 0.49V 0x3 = 0.48V 0x0 = Functional Reset"]
pub type VddOvSelfTestSelR = crate::BitReader;
#[doc = "Field `VDD_OV_SELF_TEST_SEL` writer - 9:9\\]
Enable 1.2V VDD Strict OV VMON Self Test If Self-test mode is enabled, VDD 1.2V VMON OV reference is programmed based on MSS_REFSYS_SPARE_REG&lt;23:22> and MSS_REFSYS_SPARE_REG&lt;17:16> as follows: If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.58V 0x1 = 0.57V 0x2 = 0.56V 0x3 = 0.55V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.55V 0x1 = 0.54V 0x2 = 0.53V 0x3 = 0.52V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.53V 0x1 = 0.52V 0x2 = 0.51V 0x3 = 0.5V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.51V 0x1 = 0.5V 0x2 = 0.49V 0x3 = 0.48V 0x0 = Functional Reset"]
pub type VddOvSelfTestSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA_OSC_UV_SELF_TEST_SEL` reader - 10:10\\]
Enable VDDA_OSC Strict UV VMON Self Test If Self-test mode is enabled, VDDA_OSC UV VMON reference is programmed as follows for MSS_REFSYS_SPARE_REG&lt;27:26>: 0x0 = 0.66V 0x1 = 0.64V 0x2 = 0.62V 0x3 = 0.6V 0x0 = Functional Reset"]
pub type VddaOscUvSelfTestSelR = crate::BitReader;
#[doc = "Field `VDDA_OSC_UV_SELF_TEST_SEL` writer - 10:10\\]
Enable VDDA_OSC Strict UV VMON Self Test If Self-test mode is enabled, VDDA_OSC UV VMON reference is programmed as follows for MSS_REFSYS_SPARE_REG&lt;27:26>: 0x0 = 0.66V 0x1 = 0.64V 0x2 = 0.62V 0x3 = 0.6V 0x0 = Functional Reset"]
pub type VddaOscUvSelfTestSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 11:11\\]
Reserved Reserved in case VDDA_OSC OV VMON and self test is ever implemented 0x0 = Functional Reset"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - 11:11\\]
Reserved Reserved in case VDDA_OSC OV VMON and self test is ever implemented 0x0 = Functional Reset"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDS_3P3V_UV_SELF_TEST_SEL` reader - 12:12\\]
Enable VIOIN Strict UV VMON Self Test If Self-test mode is enabled, VIOIN UV VMON reference is programmed as follows for MSS_REFSYS_SPARE_REG&lt;25:24>: 0x0 = 0.66V 0x1 = 0.64V 0x2 = 0.62V 0x3 = 0.6V 0x0 = Functional Reset"]
pub type Vdds3p3vUvSelfTestSelR = crate::BitReader;
#[doc = "Field `VDDS_3P3V_UV_SELF_TEST_SEL` writer - 12:12\\]
Enable VIOIN Strict UV VMON Self Test If Self-test mode is enabled, VIOIN UV VMON reference is programmed as follows for MSS_REFSYS_SPARE_REG&lt;25:24>: 0x0 = 0.66V 0x1 = 0.64V 0x2 = 0.62V 0x3 = 0.6V 0x0 = Functional Reset"]
pub type Vdds3p3vUvSelfTestSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 13:13\\]
Reserved Reserved in case VIOIN OV VMON and self test is ever implemented 0x0 = Functional Reset"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - 13:13\\]
Reserved Reserved in case VIOIN OV VMON and self test is ever implemented 0x0 = Functional Reset"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_OV_IR_DROP_COMP_SEL` reader - 15:14\\]
VDD 1.2V VMON OV Reference Selection and VDD 1.2V VMON UV Self-test Reference Selection If MSS_REFSYS_SPARE_REG&lt;9> = 0x0, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;17:16> programming (normal VDD 1.2V VMON OV operation) If MSS_REFSYS_SPARE_REG&lt;8> = 0x1, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming (VDD 1.2V VMON UV Self-test operation) NOTE: MSS_REFSYS_SPARE_REG&lt;9> != MSS_REFSYS_SPARE_REG&lt;8> is invalid If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0) 0x0 = 0.68V 0x1 = 0.67V 0x2 = 0.66V 0x3 = 0.65V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1) 0x0 = 0.65V 0x1 = 0.64V 0x2 = 0.63V 0x3 = 0.62V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2) 0x0 = 0.62V 0x1 = 0.61V 0x2 = 0.6V 0x3 = 0.59V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3) 0x0 = 0.59V 0x1 = 0.58V 0x2 = 0.57V 0x3 = 0.56V 0x0 = Functional Reset"]
pub type VddOvIrDropCompSelR = crate::FieldReader;
#[doc = "Field `VDD_OV_IR_DROP_COMP_SEL` writer - 15:14\\]
VDD 1.2V VMON OV Reference Selection and VDD 1.2V VMON UV Self-test Reference Selection If MSS_REFSYS_SPARE_REG&lt;9> = 0x0, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;17:16> programming (normal VDD 1.2V VMON OV operation) If MSS_REFSYS_SPARE_REG&lt;8> = 0x1, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming (VDD 1.2V VMON UV Self-test operation) NOTE: MSS_REFSYS_SPARE_REG&lt;9> != MSS_REFSYS_SPARE_REG&lt;8> is invalid If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0) 0x0 = 0.68V 0x1 = 0.67V 0x2 = 0.66V 0x3 = 0.65V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1) 0x0 = 0.65V 0x1 = 0.64V 0x2 = 0.63V 0x3 = 0.62V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2) 0x0 = 0.62V 0x1 = 0.61V 0x2 = 0.6V 0x3 = 0.59V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3) 0x0 = 0.59V 0x1 = 0.58V 0x2 = 0.57V 0x3 = 0.56V 0x0 = Functional Reset"]
pub type VddOvIrDropCompSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VDD_OV_SR_SEL` reader - 17:16\\]
Final level of VDD 1.2V VMON OV Reference Selection See definition in MSS_REFSYS_SPARE_REG&lt;15:14> for normal operation (MSS_REFSYS_SPARE_REG&lt;9> = 0x0) See definition in MSS_REFSYS_SPARE_REG&lt;23:22> for self-test operation (MSS_REFSYS_SPARE_REG&lt;9> = 0x1) 0x0 = Functional Reset"]
pub type VddOvSrSelR = crate::FieldReader;
#[doc = "Field `VDD_OV_SR_SEL` writer - 17:16\\]
Final level of VDD 1.2V VMON OV Reference Selection See definition in MSS_REFSYS_SPARE_REG&lt;15:14> for normal operation (MSS_REFSYS_SPARE_REG&lt;9> = 0x0) See definition in MSS_REFSYS_SPARE_REG&lt;23:22> for self-test operation (MSS_REFSYS_SPARE_REG&lt;9> = 0x1) 0x0 = Functional Reset"]
pub type VddOvSrSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VIOIN_UV_RSET_MASK` reader - 18:18\\]
If asserted, VIOIN_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
pub type VioinUvRsetMaskR = crate::BitReader;
#[doc = "Field `VIOIN_UV_RSET_MASK` writer - 18:18\\]
If asserted, VIOIN_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
pub type VioinUvRsetMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA_OSC_UV_RSET_MASK` reader - 19:19\\]
If asserted, VDDA_OSC_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
pub type VddaOscUvRsetMaskR = crate::BitReader;
#[doc = "Field `VDDA_OSC_UV_RSET_MASK` writer - 19:19\\]
If asserted, VDDA_OSC_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
pub type VddaOscUvRsetMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_UV_RSET_MASK` reader - 20:20\\]
If asserted, VDD_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
pub type VddUvRsetMaskR = crate::BitReader;
#[doc = "Field `VDD_UV_RSET_MASK` writer - 20:20\\]
If asserted, VDD_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
pub type VddUvRsetMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_OV_RSET_MASK` reader - 21:21\\]
If asserted, VDD_OV will not trigger the automatic reset of the device through WU Seq hardware control. However, OV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
pub type VddOvRsetMaskR = crate::BitReader;
#[doc = "Field `VDD_OV_RSET_MASK` writer - 21:21\\]
If asserted, VDD_OV will not trigger the automatic reset of the device through WU Seq hardware control. However, OV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
pub type VddOvRsetMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_IR_DROP_COMP_SEL` reader - 23:22\\]
VDD 1.2V VMON UV Reference Selection and VDD 1.2V VMON OV Self-test Reference Selection If MSS_REFSYS_SPARE_REG&lt;8> = 0x0, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming (normal VDD 1.2V VMON UV operation) If MSS_REFSYS_SPARE_REG&lt;9> = 0x1, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;17:16> programming (VDD 1.2V VMON OV Self-test operation) NOTE: MSS_REFSYS_SPARE_REG&lt;9> != MSS_REFSYS_SPARE_REG&lt;8> is invalid Reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0) 0x0 = 0.58V 0x1 = 0.57V 0x2 = 0.56V 0x3 = 0.55V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1) 0x0 = 0.55V 0x1 = 0.54V 0x2 = 0.53V 0x3 = 0.52V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2) 0x0 = 0.53V 0x1 = 0.52V 0x2 = 0.51V 0x3 = 0.5V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3) 0x0 = 0.51V 0x1 = 0.5V 0x2 = 0.49V 0x3 = 0.48V 0x0 = Functional Reset"]
pub type VddIrDropCompSelR = crate::FieldReader;
#[doc = "Field `VDD_IR_DROP_COMP_SEL` writer - 23:22\\]
VDD 1.2V VMON UV Reference Selection and VDD 1.2V VMON OV Self-test Reference Selection If MSS_REFSYS_SPARE_REG&lt;8> = 0x0, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming (normal VDD 1.2V VMON UV operation) If MSS_REFSYS_SPARE_REG&lt;9> = 0x1, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;17:16> programming (VDD 1.2V VMON OV Self-test operation) NOTE: MSS_REFSYS_SPARE_REG&lt;9> != MSS_REFSYS_SPARE_REG&lt;8> is invalid Reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0) 0x0 = 0.58V 0x1 = 0.57V 0x2 = 0.56V 0x3 = 0.55V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1) 0x0 = 0.55V 0x1 = 0.54V 0x2 = 0.53V 0x3 = 0.52V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2) 0x0 = 0.53V 0x1 = 0.52V 0x2 = 0.51V 0x3 = 0.5V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3) 0x0 = 0.51V 0x1 = 0.5V 0x2 = 0.49V 0x3 = 0.48V 0x0 = Functional Reset"]
pub type VddIrDropCompSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VDDS_3P3V_IR_DROP_COMP_SEL` reader - 25:24\\]
VIOIN VMON UV Reference Selection 0x0 = 0.56V 0x1 = 0.54V 0x2 = 0.52V 0x3 = 0.5V 0x0 = Functional Reset"]
pub type Vdds3p3vIrDropCompSelR = crate::FieldReader;
#[doc = "Field `VDDS_3P3V_IR_DROP_COMP_SEL` writer - 25:24\\]
VIOIN VMON UV Reference Selection 0x0 = 0.56V 0x1 = 0.54V 0x2 = 0.52V 0x3 = 0.5V 0x0 = Functional Reset"]
pub type Vdds3p3vIrDropCompSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VDDA_OSC_IR_DROP_COMP_SEL` reader - 27:26\\]
VDDA_OSC UV VMON Reference Selection 0x0 = 0.56V 0x1 = 0.54V 0x2 = 0.52V 0x3 = 0.5V 0x0 = Functional Reset"]
pub type VddaOscIrDropCompSelR = crate::FieldReader;
#[doc = "Field `VDDA_OSC_IR_DROP_COMP_SEL` writer - 27:26\\]
VDDA_OSC UV VMON Reference Selection 0x0 = 0.56V 0x1 = 0.54V 0x2 = 0.52V 0x3 = 0.5V 0x0 = Functional Reset"]
pub type VddaOscIrDropCompSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED5` reader - 30:28\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 30:28\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED6` reader - 31:31\\]
Reserved Used for ANALOGTEST TMUX ESD CTRL in Pad-Frame in TPR (formerly RX_REFSYS_TMUX_SPARE_CTRL_LOWV&lt;31> in AWR/IWR devices, but RX does not exist in TPR) 0x0 = Functional Reset"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `RESERVED6` writer - 31:31\\]
Reserved Used for ANALOGTEST TMUX ESD CTRL in Pad-Frame in TPR (formerly RX_REFSYS_TMUX_SPARE_CTRL_LOWV&lt;31> in AWR/IWR devices, but RX does not exist in TPR) 0x0 = Functional Reset"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved In TPR, this bit is mapped to efuse and is used to control POR_DIG_SEQ_ECO_DIS. Writes have no effect. 0: Enable Slicer delay ECO 1: Disable Slicer delay ECO 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - 5:1\\]
Reserved Reads return 0x0 and writes have no effect, but these bits are tied to efuse overrides. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Final level of VDD 1.2V VMON UV Reference Selection See definition in MSS_REFSYS_SPARE_REG&lt;23:22> for normal operation (MSS_REFSYS_SPARE_REG&lt;8> = 0x0) See definition in MSS_REFSYS_SPARE_REG&lt;15:14> for self-test operation (MSS_REFSYS_SPARE_REG&lt;8> = 0x1) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdd_sr_sel(&self) -> VddSrSelR {
        VddSrSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable 1.2V VDD Strict UV VMON Self Test If Self-test mode is enabled, VDD 1.2V VMON UV reference is programmed based on MSS_REFSYS_SPARE_REG&lt;15:14> and MSS_REFSYS_SPARE_REG&lt;7:6> as follows: If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.68V 0x1 = 0.67V 0x2 = 0.66V 0x3 = 0.65V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.65V 0x1 = 0.64V 0x2 = 0.63V 0x3 = 0.62V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.62V 0x1 = 0.61V 0x2 = 0.6V 0x3 = 0.59V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.59V 0x1 = 0.58V 0x2 = 0.57V 0x3 = 0.56V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdd_uv_self_test_sel(&self) -> VddUvSelfTestSelR {
        VddUvSelfTestSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable 1.2V VDD Strict OV VMON Self Test If Self-test mode is enabled, VDD 1.2V VMON OV reference is programmed based on MSS_REFSYS_SPARE_REG&lt;23:22> and MSS_REFSYS_SPARE_REG&lt;17:16> as follows: If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.58V 0x1 = 0.57V 0x2 = 0.56V 0x3 = 0.55V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.55V 0x1 = 0.54V 0x2 = 0.53V 0x3 = 0.52V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.53V 0x1 = 0.52V 0x2 = 0.51V 0x3 = 0.5V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.51V 0x1 = 0.5V 0x2 = 0.49V 0x3 = 0.48V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdd_ov_self_test_sel(&self) -> VddOvSelfTestSelR {
        VddOvSelfTestSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable VDDA_OSC Strict UV VMON Self Test If Self-test mode is enabled, VDDA_OSC UV VMON reference is programmed as follows for MSS_REFSYS_SPARE_REG&lt;27:26>: 0x0 = 0.66V 0x1 = 0.64V 0x2 = 0.62V 0x3 = 0.6V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdda_osc_uv_self_test_sel(&self) -> VddaOscUvSelfTestSelR {
        VddaOscUvSelfTestSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Reserved Reserved in case VDDA_OSC OV VMON and self test is ever implemented 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable VIOIN Strict UV VMON Self Test If Self-test mode is enabled, VIOIN UV VMON reference is programmed as follows for MSS_REFSYS_SPARE_REG&lt;25:24>: 0x0 = 0.66V 0x1 = 0.64V 0x2 = 0.62V 0x3 = 0.6V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdds_3p3v_uv_self_test_sel(&self) -> Vdds3p3vUvSelfTestSelR {
        Vdds3p3vUvSelfTestSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Reserved Reserved in case VIOIN OV VMON and self test is ever implemented 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
VDD 1.2V VMON OV Reference Selection and VDD 1.2V VMON UV Self-test Reference Selection If MSS_REFSYS_SPARE_REG&lt;9> = 0x0, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;17:16> programming (normal VDD 1.2V VMON OV operation) If MSS_REFSYS_SPARE_REG&lt;8> = 0x1, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming (VDD 1.2V VMON UV Self-test operation) NOTE: MSS_REFSYS_SPARE_REG&lt;9> != MSS_REFSYS_SPARE_REG&lt;8> is invalid If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0) 0x0 = 0.68V 0x1 = 0.67V 0x2 = 0.66V 0x3 = 0.65V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1) 0x0 = 0.65V 0x1 = 0.64V 0x2 = 0.63V 0x3 = 0.62V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2) 0x0 = 0.62V 0x1 = 0.61V 0x2 = 0.6V 0x3 = 0.59V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3) 0x0 = 0.59V 0x1 = 0.58V 0x2 = 0.57V 0x3 = 0.56V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdd_ov_ir_drop_comp_sel(&self) -> VddOvIrDropCompSelR {
        VddOvIrDropCompSelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Final level of VDD 1.2V VMON OV Reference Selection See definition in MSS_REFSYS_SPARE_REG&lt;15:14> for normal operation (MSS_REFSYS_SPARE_REG&lt;9> = 0x0) See definition in MSS_REFSYS_SPARE_REG&lt;23:22> for self-test operation (MSS_REFSYS_SPARE_REG&lt;9> = 0x1) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdd_ov_sr_sel(&self) -> VddOvSrSelR {
        VddOvSrSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
If asserted, VIOIN_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vioin_uv_rset_mask(&self) -> VioinUvRsetMaskR {
        VioinUvRsetMaskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
If asserted, VDDA_OSC_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdda_osc_uv_rset_mask(&self) -> VddaOscUvRsetMaskR {
        VddaOscUvRsetMaskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
If asserted, VDD_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdd_uv_rset_mask(&self) -> VddUvRsetMaskR {
        VddUvRsetMaskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
If asserted, VDD_OV will not trigger the automatic reset of the device through WU Seq hardware control. However, OV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdd_ov_rset_mask(&self) -> VddOvRsetMaskR {
        VddOvRsetMaskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
VDD 1.2V VMON UV Reference Selection and VDD 1.2V VMON OV Self-test Reference Selection If MSS_REFSYS_SPARE_REG&lt;8> = 0x0, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming (normal VDD 1.2V VMON UV operation) If MSS_REFSYS_SPARE_REG&lt;9> = 0x1, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;17:16> programming (VDD 1.2V VMON OV Self-test operation) NOTE: MSS_REFSYS_SPARE_REG&lt;9> != MSS_REFSYS_SPARE_REG&lt;8> is invalid Reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0) 0x0 = 0.58V 0x1 = 0.57V 0x2 = 0.56V 0x3 = 0.55V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1) 0x0 = 0.55V 0x1 = 0.54V 0x2 = 0.53V 0x3 = 0.52V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2) 0x0 = 0.53V 0x1 = 0.52V 0x2 = 0.51V 0x3 = 0.5V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3) 0x0 = 0.51V 0x1 = 0.5V 0x2 = 0.49V 0x3 = 0.48V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdd_ir_drop_comp_sel(&self) -> VddIrDropCompSelR {
        VddIrDropCompSelR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
VIOIN VMON UV Reference Selection 0x0 = 0.56V 0x1 = 0.54V 0x2 = 0.52V 0x3 = 0.5V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdds_3p3v_ir_drop_comp_sel(&self) -> Vdds3p3vIrDropCompSelR {
        Vdds3p3vIrDropCompSelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 27:26\\]
VDDA_OSC UV VMON Reference Selection 0x0 = 0.56V 0x1 = 0.54V 0x2 = 0.52V 0x3 = 0.5V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdda_osc_ir_drop_comp_sel(&self) -> VddaOscIrDropCompSelR {
        VddaOscIrDropCompSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved Used for ANALOGTEST TMUX ESD CTRL in Pad-Frame in TPR (formerly RX_REFSYS_TMUX_SPARE_CTRL_LOWV&lt;31> in AWR/IWR devices, but RX does not exist in TPR) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved In TPR, this bit is mapped to efuse and is used to control POR_DIG_SEQ_ECO_DIS. Writes have no effect. 0: Enable Slicer delay ECO 1: Disable Slicer delay ECO 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegRefsysSpareRegLowvSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 1:5 - 5:1\\]
Reserved Reads return 0x0 and writes have no effect, but these bits are tied to efuse overrides. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<AnaRegRefsysSpareRegLowvSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Final level of VDD 1.2V VMON UV Reference Selection See definition in MSS_REFSYS_SPARE_REG&lt;23:22> for normal operation (MSS_REFSYS_SPARE_REG&lt;8> = 0x0) See definition in MSS_REFSYS_SPARE_REG&lt;15:14> for self-test operation (MSS_REFSYS_SPARE_REG&lt;8> = 0x1) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_sr_sel(&mut self) -> VddSrSelW<AnaRegRefsysSpareRegLowvSpec> {
        VddSrSelW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable 1.2V VDD Strict UV VMON Self Test If Self-test mode is enabled, VDD 1.2V VMON UV reference is programmed based on MSS_REFSYS_SPARE_REG&lt;15:14> and MSS_REFSYS_SPARE_REG&lt;7:6> as follows: If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.68V 0x1 = 0.67V 0x2 = 0.66V 0x3 = 0.65V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.65V 0x1 = 0.64V 0x2 = 0.63V 0x3 = 0.62V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.62V 0x1 = 0.61V 0x2 = 0.6V 0x3 = 0.59V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3, MSS_REFSYS_SPARE_REG&lt;15:14>: 0x0 = 0.59V 0x1 = 0.58V 0x2 = 0.57V 0x3 = 0.56V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_uv_self_test_sel(&mut self) -> VddUvSelfTestSelW<AnaRegRefsysSpareRegLowvSpec> {
        VddUvSelfTestSelW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable 1.2V VDD Strict OV VMON Self Test If Self-test mode is enabled, VDD 1.2V VMON OV reference is programmed based on MSS_REFSYS_SPARE_REG&lt;23:22> and MSS_REFSYS_SPARE_REG&lt;17:16> as follows: If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.58V 0x1 = 0.57V 0x2 = 0.56V 0x3 = 0.55V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.55V 0x1 = 0.54V 0x2 = 0.53V 0x3 = 0.52V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.53V 0x1 = 0.52V 0x2 = 0.51V 0x3 = 0.5V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3, MSS_REFSYS_SPARE_REG&lt;23:22>: 0x0 = 0.51V 0x1 = 0.5V 0x2 = 0.49V 0x3 = 0.48V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_ov_self_test_sel(&mut self) -> VddOvSelfTestSelW<AnaRegRefsysSpareRegLowvSpec> {
        VddOvSelfTestSelW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable VDDA_OSC Strict UV VMON Self Test If Self-test mode is enabled, VDDA_OSC UV VMON reference is programmed as follows for MSS_REFSYS_SPARE_REG&lt;27:26>: 0x0 = 0.66V 0x1 = 0.64V 0x2 = 0.62V 0x3 = 0.6V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdda_osc_uv_self_test_sel(
        &mut self,
    ) -> VddaOscUvSelfTestSelW<AnaRegRefsysSpareRegLowvSpec> {
        VddaOscUvSelfTestSelW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Reserved Reserved in case VDDA_OSC OV VMON and self test is ever implemented 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<AnaRegRefsysSpareRegLowvSpec> {
        Reserved2W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable VIOIN Strict UV VMON Self Test If Self-test mode is enabled, VIOIN UV VMON reference is programmed as follows for MSS_REFSYS_SPARE_REG&lt;25:24>: 0x0 = 0.66V 0x1 = 0.64V 0x2 = 0.62V 0x3 = 0.6V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdds_3p3v_uv_self_test_sel(
        &mut self,
    ) -> Vdds3p3vUvSelfTestSelW<AnaRegRefsysSpareRegLowvSpec> {
        Vdds3p3vUvSelfTestSelW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Reserved Reserved in case VIOIN OV VMON and self test is ever implemented 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<AnaRegRefsysSpareRegLowvSpec> {
        Reserved3W::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
VDD 1.2V VMON OV Reference Selection and VDD 1.2V VMON UV Self-test Reference Selection If MSS_REFSYS_SPARE_REG&lt;9> = 0x0, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;17:16> programming (normal VDD 1.2V VMON OV operation) If MSS_REFSYS_SPARE_REG&lt;8> = 0x1, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming (VDD 1.2V VMON UV Self-test operation) NOTE: MSS_REFSYS_SPARE_REG&lt;9> != MSS_REFSYS_SPARE_REG&lt;8> is invalid If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0) 0x0 = 0.68V 0x1 = 0.67V 0x2 = 0.66V 0x3 = 0.65V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1) 0x0 = 0.65V 0x1 = 0.64V 0x2 = 0.63V 0x3 = 0.62V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2) 0x0 = 0.62V 0x1 = 0.61V 0x2 = 0.6V 0x3 = 0.59V If MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3 (or MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3) 0x0 = 0.59V 0x1 = 0.58V 0x2 = 0.57V 0x3 = 0.56V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_ov_ir_drop_comp_sel(&mut self) -> VddOvIrDropCompSelW<AnaRegRefsysSpareRegLowvSpec> {
        VddOvIrDropCompSelW::new(self, 14)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Final level of VDD 1.2V VMON OV Reference Selection See definition in MSS_REFSYS_SPARE_REG&lt;15:14> for normal operation (MSS_REFSYS_SPARE_REG&lt;9> = 0x0) See definition in MSS_REFSYS_SPARE_REG&lt;23:22> for self-test operation (MSS_REFSYS_SPARE_REG&lt;9> = 0x1) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_ov_sr_sel(&mut self) -> VddOvSrSelW<AnaRegRefsysSpareRegLowvSpec> {
        VddOvSrSelW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
If asserted, VIOIN_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vioin_uv_rset_mask(&mut self) -> VioinUvRsetMaskW<AnaRegRefsysSpareRegLowvSpec> {
        VioinUvRsetMaskW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
If asserted, VDDA_OSC_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdda_osc_uv_rset_mask(&mut self) -> VddaOscUvRsetMaskW<AnaRegRefsysSpareRegLowvSpec> {
        VddaOscUvRsetMaskW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
If asserted, VDD_UV will not trigger the automatic reset of the device through WU Seq hardware control. However, UV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_uv_rset_mask(&mut self) -> VddUvRsetMaskW<AnaRegRefsysSpareRegLowvSpec> {
        VddUvRsetMaskW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
If asserted, VDD_OV will not trigger the automatic reset of the device through WU Seq hardware control. However, OV flag will still propagate to the digital where the CPU will need to take action. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_ov_rset_mask(&mut self) -> VddOvRsetMaskW<AnaRegRefsysSpareRegLowvSpec> {
        VddOvRsetMaskW::new(self, 21)
    }
    #[doc = "Bits 22:23 - 23:22\\]
VDD 1.2V VMON UV Reference Selection and VDD 1.2V VMON OV Self-test Reference Selection If MSS_REFSYS_SPARE_REG&lt;8> = 0x0, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming (normal VDD 1.2V VMON UV operation) If MSS_REFSYS_SPARE_REG&lt;9> = 0x1, reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;17:16> programming (VDD 1.2V VMON OV Self-test operation) NOTE: MSS_REFSYS_SPARE_REG&lt;9> != MSS_REFSYS_SPARE_REG&lt;8> is invalid Reference selection is dependent on MSS_REFSYS_SPARE_REG&lt;7:6> programming If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x0 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x0) 0x0 = 0.58V 0x1 = 0.57V 0x2 = 0.56V 0x3 = 0.55V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x1 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x1) 0x0 = 0.55V 0x1 = 0.54V 0x2 = 0.53V 0x3 = 0.52V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x2 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x2) 0x0 = 0.53V 0x1 = 0.52V 0x2 = 0.51V 0x3 = 0.5V If MSS_REFSYS_SPARE_REG&lt;7:6> = 0x3 (or MSS_REFSYS_SPARE_REG&lt;17:16> = 0x3) 0x0 = 0.51V 0x1 = 0.5V 0x2 = 0.49V 0x3 = 0.48V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_ir_drop_comp_sel(&mut self) -> VddIrDropCompSelW<AnaRegRefsysSpareRegLowvSpec> {
        VddIrDropCompSelW::new(self, 22)
    }
    #[doc = "Bits 24:25 - 25:24\\]
VIOIN VMON UV Reference Selection 0x0 = 0.56V 0x1 = 0.54V 0x2 = 0.52V 0x3 = 0.5V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdds_3p3v_ir_drop_comp_sel(
        &mut self,
    ) -> Vdds3p3vIrDropCompSelW<AnaRegRefsysSpareRegLowvSpec> {
        Vdds3p3vIrDropCompSelW::new(self, 24)
    }
    #[doc = "Bits 26:27 - 27:26\\]
VDDA_OSC UV VMON Reference Selection 0x0 = 0.56V 0x1 = 0.54V 0x2 = 0.52V 0x3 = 0.5V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdda_osc_ir_drop_comp_sel(
        &mut self,
    ) -> VddaOscIrDropCompSelW<AnaRegRefsysSpareRegLowvSpec> {
        VddaOscIrDropCompSelW::new(self, 26)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<AnaRegRefsysSpareRegLowvSpec> {
        Reserved5W::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved Used for ANALOGTEST TMUX ESD CTRL in Pad-Frame in TPR (formerly RX_REFSYS_TMUX_SPARE_CTRL_LOWV&lt;31> in AWR/IWR devices, but RX does not exist in TPR) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<AnaRegRefsysSpareRegLowvSpec> {
        Reserved6W::new(self, 31)
    }
}
#[doc = "ANA_REG_REFSYS_SPARE_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_refsys_spare_reg_lowv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_refsys_spare_reg_lowv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegRefsysSpareRegLowvSpec;
impl crate::RegisterSpec for AnaRegRefsysSpareRegLowvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_refsys_spare_reg_lowv::R`](R) reader structure"]
impl crate::Readable for AnaRegRefsysSpareRegLowvSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_refsys_spare_reg_lowv::W`](W) writer structure"]
impl crate::Writable for AnaRegRefsysSpareRegLowvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_REFSYS_SPARE_REG_LOWV to value 0"]
impl crate::Resettable for AnaRegRefsysSpareRegLowvSpec {
    const RESET_VALUE: u32 = 0;
}
