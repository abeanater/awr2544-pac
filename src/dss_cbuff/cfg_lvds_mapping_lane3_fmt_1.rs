#[doc = "Register `CFG_LVDS_MAPPING_LANE3_FMT_1` reader"]
pub type R = crate::R<CfgLvdsMappingLane3Fmt1Spec>;
#[doc = "Register `CFG_LVDS_MAPPING_LANE3_FMT_1` writer"]
pub type W = crate::W<CfgLvdsMappingLane3Fmt1Spec>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_A` reader - 3:0\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1AR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_A` writer - 3:0\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1AW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_B` reader - 7:4\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1BR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_B` writer - 7:4\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1BW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_C` reader - 11:8\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1CR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_C` writer - 11:8\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1CW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_D` reader - 15:12\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1DR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_D` writer - 15:12\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1DW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_E` reader - 19:16\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1ER = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_E` writer - 19:16\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1EW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_F` reader - 23:20\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1FR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_F` writer - 23:20\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1FW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_G` reader - 27:24\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1GR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_G` writer - 27:24\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1GW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_H` reader - 31:28\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1HR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE3_FMT_1_H` writer - 31:28\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane3Fmt1HW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane3_fmt_1_a(&self) -> CfgLvdsMappingLane3Fmt1AR {
        CfgLvdsMappingLane3Fmt1AR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane3_fmt_1_b(&self) -> CfgLvdsMappingLane3Fmt1BR {
        CfgLvdsMappingLane3Fmt1BR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane3_fmt_1_c(&self) -> CfgLvdsMappingLane3Fmt1CR {
        CfgLvdsMappingLane3Fmt1CR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane3_fmt_1_d(&self) -> CfgLvdsMappingLane3Fmt1DR {
        CfgLvdsMappingLane3Fmt1DR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane3_fmt_1_e(&self) -> CfgLvdsMappingLane3Fmt1ER {
        CfgLvdsMappingLane3Fmt1ER::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane3_fmt_1_f(&self) -> CfgLvdsMappingLane3Fmt1FR {
        CfgLvdsMappingLane3Fmt1FR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane3_fmt_1_g(&self) -> CfgLvdsMappingLane3Fmt1GR {
        CfgLvdsMappingLane3Fmt1GR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane3_fmt_1_h(&self) -> CfgLvdsMappingLane3Fmt1HR {
        CfgLvdsMappingLane3Fmt1HR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane3_fmt_1_a(
        &mut self,
    ) -> CfgLvdsMappingLane3Fmt1AW<CfgLvdsMappingLane3Fmt1Spec> {
        CfgLvdsMappingLane3Fmt1AW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane3_fmt_1_b(
        &mut self,
    ) -> CfgLvdsMappingLane3Fmt1BW<CfgLvdsMappingLane3Fmt1Spec> {
        CfgLvdsMappingLane3Fmt1BW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane3_fmt_1_c(
        &mut self,
    ) -> CfgLvdsMappingLane3Fmt1CW<CfgLvdsMappingLane3Fmt1Spec> {
        CfgLvdsMappingLane3Fmt1CW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane3_fmt_1_d(
        &mut self,
    ) -> CfgLvdsMappingLane3Fmt1DW<CfgLvdsMappingLane3Fmt1Spec> {
        CfgLvdsMappingLane3Fmt1DW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane3_fmt_1_e(
        &mut self,
    ) -> CfgLvdsMappingLane3Fmt1EW<CfgLvdsMappingLane3Fmt1Spec> {
        CfgLvdsMappingLane3Fmt1EW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane3_fmt_1_f(
        &mut self,
    ) -> CfgLvdsMappingLane3Fmt1FW<CfgLvdsMappingLane3Fmt1Spec> {
        CfgLvdsMappingLane3Fmt1FW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane3_fmt_1_g(
        &mut self,
    ) -> CfgLvdsMappingLane3Fmt1GW<CfgLvdsMappingLane3Fmt1Spec> {
        CfgLvdsMappingLane3Fmt1GW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane3_fmt_1_h(
        &mut self,
    ) -> CfgLvdsMappingLane3Fmt1HW<CfgLvdsMappingLane3Fmt1Spec> {
        CfgLvdsMappingLane3Fmt1HW::new(self, 28)
    }
}
#[doc = "CFG_LVDS_MAPPING_LANE3_FMT_1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_mapping_lane3_fmt_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_mapping_lane3_fmt_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgLvdsMappingLane3Fmt1Spec;
impl crate::RegisterSpec for CfgLvdsMappingLane3Fmt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_lvds_mapping_lane3_fmt_1::R`](R) reader structure"]
impl crate::Readable for CfgLvdsMappingLane3Fmt1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_lvds_mapping_lane3_fmt_1::W`](W) writer structure"]
impl crate::Writable for CfgLvdsMappingLane3Fmt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_LVDS_MAPPING_LANE3_FMT_1 to value 0"]
impl crate::Resettable for CfgLvdsMappingLane3Fmt1Spec {
    const RESET_VALUE: u32 = 0;
}
