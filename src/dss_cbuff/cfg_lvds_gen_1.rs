#[doc = "Register `CFG_LVDS_GEN_1` reader"]
pub type R = crate::R<CfgLvdsGen1Spec>;
#[doc = "Register `CFG_LVDS_GEN_1` writer"]
pub type W = crate::W<CfgLvdsGen1Spec>;
#[doc = "Field `ctpen` reader - 0:0\\]
TI Internal feature. 0 : Regular Operation 1 : LVDS Testpattern Enable"]
pub type CtpenR = crate::BitReader;
#[doc = "Field `ctpen` writer - 0:0\\]
TI Internal feature. 0 : Regular Operation 1 : LVDS Testpattern Enable"]
pub type CtpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `csdrinv` reader - 1:1\\]
TI Internal feature. Configure the clock inversion during SDR mode. 0 : No inversion 1 : Inversion"]
pub type CsdrinvR = crate::BitReader;
#[doc = "Field `csdrinv` writer - 1:1\\]
TI Internal feature. Configure the clock inversion during SDR mode. 0 : No inversion 1 : Inversion"]
pub type CsdrinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `c3c3l` reader - 2:2\\]
LVDS Only Programming: 0 : Regular Operation 1 : Enable 3Ch-3Lane mode in LVDS. Refer to Programming model for more details"]
pub type C3c3lR = crate::BitReader;
#[doc = "Field `c3c3l` writer - 2:2\\]
LVDS Only Programming: 0 : Regular Operation 1 : Enable 3Ch-3Lane mode in LVDS. Refer to Programming model for more details"]
pub type C3c3lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - "]
pub type Nu3R = crate::BitReader;
#[doc = "Field `NU3` writer - "]
pub type Nu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctiddly` reader - 6:4\\]
TI Internal feature. Configure the skew delay in terms on number of cycles"]
pub type CtiddlyR = crate::FieldReader;
#[doc = "Field `ctiddly` writer - 6:4\\]
TI Internal feature. Configure the skew delay in terms on number of cycles"]
pub type CtiddlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU1` reader - 7:7\\]
RESERVED"]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - 7:7\\]
RESERVED"]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctpsel0` reader - 9:8\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 0"]
pub type Ctpsel0R = crate::FieldReader;
#[doc = "Field `ctpsel0` writer - 9:8\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 0"]
pub type Ctpsel0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ctpsel1` reader - 11:10\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 1"]
pub type Ctpsel1R = crate::FieldReader;
#[doc = "Field `ctpsel1` writer - 11:10\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 1"]
pub type Ctpsel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ctpsel2` reader - 13:12\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 2"]
pub type Ctpsel2R = crate::FieldReader;
#[doc = "Field `ctpsel2` writer - 13:12\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 2"]
pub type Ctpsel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ctpsel3` reader - 15:14\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 3"]
pub type Ctpsel3R = crate::FieldReader;
#[doc = "Field `ctpsel3` writer - 15:14\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 3"]
pub type Ctpsel3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clfven` reader - 16:16\\]
TI Internal feature. Extend the Single Ended Frame Valid When the frame_valid is used as a single ended signal, then make this 1. 0 : Regular Operation. Frame Valid will exactly match with the valid data. 1 : The frame_valid would start early by about 10 lvds_clk (internal) and would extend beyond by 10 lvds_clk (internal) after the end of the frame"]
pub type ClfvenR = crate::BitReader;
#[doc = "Field `clfven` writer - 16:16\\]
TI Internal feature. Extend the Single Ended Frame Valid When the frame_valid is used as a single ended signal, then make this 1. 0 : Regular Operation. Frame Valid will exactly match with the valid data. 1 : The frame_valid would start early by about 10 lvds_clk (internal) and would extend beyond by 10 lvds_clk (internal) after the end of the frame"]
pub type ClfvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfcpol` reader - 17:17\\]
TI Internal Feature. 0 : During IDLE, Frame clock will be 0. Start of the valid sample is indicated by the rise edge 1 : During IDLE. Frame clock will be 1. Start of the valid sample is indicated by the fall edge."]
pub type CfcpolR = crate::BitReader;
#[doc = "Field `cfcpol` writer - 17:17\\]
TI Internal Feature. 0 : During IDLE, Frame clock will be 0. Start of the valid sample is indicated by the rise edge 1 : During IDLE. Frame clock will be 1. Start of the valid sample is indicated by the fall edge."]
pub type CfcpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgbcen` reader - 18:18\\]
TI Internal Feature. 0 : Bit clk is free running 1 : Bit clk is valid only during the valid frame."]
pub type CgbcenR = crate::BitReader;
#[doc = "Field `cgbcen` writer - 18:18\\]
TI Internal Feature. 0 : Bit clk is free running 1 : Bit clk is valid only during the valid frame."]
pub type CgbcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 31:19\\]
RESERVED"]
pub type Nu2R = crate::FieldReader<u16>;
#[doc = "Field `NU2` writer - 31:19\\]
RESERVED"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TI Internal feature. 0 : Regular Operation 1 : LVDS Testpattern Enable"]
    #[inline(always)]
    pub fn ctpen(&self) -> CtpenR {
        CtpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal feature. Configure the clock inversion during SDR mode. 0 : No inversion 1 : Inversion"]
    #[inline(always)]
    pub fn csdrinv(&self) -> CsdrinvR {
        CsdrinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
LVDS Only Programming: 0 : Regular Operation 1 : Enable 3Ch-3Lane mode in LVDS. Refer to Programming model for more details"]
    #[inline(always)]
    pub fn c3c3l(&self) -> C3c3lR {
        C3c3lR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
TI Internal feature. Configure the skew delay in terms on number of cycles"]
    #[inline(always)]
    pub fn ctiddly(&self) -> CtiddlyR {
        CtiddlyR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
RESERVED"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 0"]
    #[inline(always)]
    pub fn ctpsel0(&self) -> Ctpsel0R {
        Ctpsel0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 1"]
    #[inline(always)]
    pub fn ctpsel1(&self) -> Ctpsel1R {
        Ctpsel1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 2"]
    #[inline(always)]
    pub fn ctpsel2(&self) -> Ctpsel2R {
        Ctpsel2R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 3"]
    #[inline(always)]
    pub fn ctpsel3(&self) -> Ctpsel3R {
        Ctpsel3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
TI Internal feature. Extend the Single Ended Frame Valid When the frame_valid is used as a single ended signal, then make this 1. 0 : Regular Operation. Frame Valid will exactly match with the valid data. 1 : The frame_valid would start early by about 10 lvds_clk (internal) and would extend beyond by 10 lvds_clk (internal) after the end of the frame"]
    #[inline(always)]
    pub fn clfven(&self) -> ClfvenR {
        ClfvenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
TI Internal Feature. 0 : During IDLE, Frame clock will be 0. Start of the valid sample is indicated by the rise edge 1 : During IDLE. Frame clock will be 1. Start of the valid sample is indicated by the fall edge."]
    #[inline(always)]
    pub fn cfcpol(&self) -> CfcpolR {
        CfcpolR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
TI Internal Feature. 0 : Bit clk is free running 1 : Bit clk is valid only during the valid frame."]
    #[inline(always)]
    pub fn cgbcen(&self) -> CgbcenR {
        CgbcenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - 31:19\\]
RESERVED"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TI Internal feature. 0 : Regular Operation 1 : LVDS Testpattern Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctpen(&mut self) -> CtpenW<CfgLvdsGen1Spec> {
        CtpenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal feature. Configure the clock inversion during SDR mode. 0 : No inversion 1 : Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn csdrinv(&mut self) -> CsdrinvW<CfgLvdsGen1Spec> {
        CsdrinvW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
LVDS Only Programming: 0 : Regular Operation 1 : Enable 3Ch-3Lane mode in LVDS. Refer to Programming model for more details"]
    #[inline(always)]
    #[must_use]
    pub fn c3c3l(&mut self) -> C3c3lW<CfgLvdsGen1Spec> {
        C3c3lW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<CfgLvdsGen1Spec> {
        Nu3W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
TI Internal feature. Configure the skew delay in terms on number of cycles"]
    #[inline(always)]
    #[must_use]
    pub fn ctiddly(&mut self) -> CtiddlyW<CfgLvdsGen1Spec> {
        CtiddlyW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<CfgLvdsGen1Spec> {
        Nu1W::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctpsel0(&mut self) -> Ctpsel0W<CfgLvdsGen1Spec> {
        Ctpsel0W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctpsel1(&mut self) -> Ctpsel1W<CfgLvdsGen1Spec> {
        Ctpsel1W::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctpsel2(&mut self) -> Ctpsel2W<CfgLvdsGen1Spec> {
        Ctpsel2W::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
TI Internal feature. This is used when Test Pattern Generation Enabled is enabled. 0 :Incremental pattern - For Lane 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctpsel3(&mut self) -> Ctpsel3W<CfgLvdsGen1Spec> {
        Ctpsel3W::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
TI Internal feature. Extend the Single Ended Frame Valid When the frame_valid is used as a single ended signal, then make this 1. 0 : Regular Operation. Frame Valid will exactly match with the valid data. 1 : The frame_valid would start early by about 10 lvds_clk (internal) and would extend beyond by 10 lvds_clk (internal) after the end of the frame"]
    #[inline(always)]
    #[must_use]
    pub fn clfven(&mut self) -> ClfvenW<CfgLvdsGen1Spec> {
        ClfvenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
TI Internal Feature. 0 : During IDLE, Frame clock will be 0. Start of the valid sample is indicated by the rise edge 1 : During IDLE. Frame clock will be 1. Start of the valid sample is indicated by the fall edge."]
    #[inline(always)]
    #[must_use]
    pub fn cfcpol(&mut self) -> CfcpolW<CfgLvdsGen1Spec> {
        CfcpolW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
TI Internal Feature. 0 : Bit clk is free running 1 : Bit clk is valid only during the valid frame."]
    #[inline(always)]
    #[must_use]
    pub fn cgbcen(&mut self) -> CgbcenW<CfgLvdsGen1Spec> {
        CgbcenW::new(self, 18)
    }
    #[doc = "Bits 19:31 - 31:19\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<CfgLvdsGen1Spec> {
        Nu2W::new(self, 19)
    }
}
#[doc = "CFG_LVDS_GEN_1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_gen_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_gen_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgLvdsGen1Spec;
impl crate::RegisterSpec for CfgLvdsGen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_lvds_gen_1::R`](R) reader structure"]
impl crate::Readable for CfgLvdsGen1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_lvds_gen_1::W`](W) writer structure"]
impl crate::Writable for CfgLvdsGen1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_LVDS_GEN_1 to value 0"]
impl crate::Resettable for CfgLvdsGen1Spec {
    const RESET_VALUE: u32 = 0;
}
