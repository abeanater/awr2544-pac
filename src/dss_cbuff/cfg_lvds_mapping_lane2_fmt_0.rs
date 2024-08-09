#[doc = "Register `CFG_LVDS_MAPPING_LANE2_FMT_0` reader"]
pub type R = crate::R<CfgLvdsMappingLane2Fmt0Spec>;
#[doc = "Register `CFG_LVDS_MAPPING_LANE2_FMT_0` writer"]
pub type W = crate::W<CfgLvdsMappingLane2Fmt0Spec>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_A` reader - 3:0\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0AR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_A` writer - 3:0\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0AW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_B` reader - 7:4\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0BR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_B` writer - 7:4\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0BW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_C` reader - 11:8\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0CR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_C` writer - 11:8\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0CW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_D` reader - 15:12\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0DR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_D` writer - 15:12\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0DW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_E` reader - 19:16\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0ER = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_E` writer - 19:16\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0EW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_F` reader - 23:20\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0FR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_F` writer - 23:20\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0FW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_G` reader - 27:24\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0GR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_G` writer - 27:24\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0GW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_H` reader - 31:28\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0HR = crate::FieldReader;
#[doc = "Field `CFG_LVDS_MAPPING_LANE2_FMT_0_H` writer - 31:28\\]
Please refer to LVDS Mapping Format section for confiuration details"]
pub type CfgLvdsMappingLane2Fmt0HW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane2_fmt_0_a(&self) -> CfgLvdsMappingLane2Fmt0AR {
        CfgLvdsMappingLane2Fmt0AR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane2_fmt_0_b(&self) -> CfgLvdsMappingLane2Fmt0BR {
        CfgLvdsMappingLane2Fmt0BR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane2_fmt_0_c(&self) -> CfgLvdsMappingLane2Fmt0CR {
        CfgLvdsMappingLane2Fmt0CR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane2_fmt_0_d(&self) -> CfgLvdsMappingLane2Fmt0DR {
        CfgLvdsMappingLane2Fmt0DR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane2_fmt_0_e(&self) -> CfgLvdsMappingLane2Fmt0ER {
        CfgLvdsMappingLane2Fmt0ER::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane2_fmt_0_f(&self) -> CfgLvdsMappingLane2Fmt0FR {
        CfgLvdsMappingLane2Fmt0FR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane2_fmt_0_g(&self) -> CfgLvdsMappingLane2Fmt0GR {
        CfgLvdsMappingLane2Fmt0GR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    pub fn cfg_lvds_mapping_lane2_fmt_0_h(&self) -> CfgLvdsMappingLane2Fmt0HR {
        CfgLvdsMappingLane2Fmt0HR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane2_fmt_0_a(
        &mut self,
    ) -> CfgLvdsMappingLane2Fmt0AW<CfgLvdsMappingLane2Fmt0Spec> {
        CfgLvdsMappingLane2Fmt0AW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane2_fmt_0_b(
        &mut self,
    ) -> CfgLvdsMappingLane2Fmt0BW<CfgLvdsMappingLane2Fmt0Spec> {
        CfgLvdsMappingLane2Fmt0BW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane2_fmt_0_c(
        &mut self,
    ) -> CfgLvdsMappingLane2Fmt0CW<CfgLvdsMappingLane2Fmt0Spec> {
        CfgLvdsMappingLane2Fmt0CW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane2_fmt_0_d(
        &mut self,
    ) -> CfgLvdsMappingLane2Fmt0DW<CfgLvdsMappingLane2Fmt0Spec> {
        CfgLvdsMappingLane2Fmt0DW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane2_fmt_0_e(
        &mut self,
    ) -> CfgLvdsMappingLane2Fmt0EW<CfgLvdsMappingLane2Fmt0Spec> {
        CfgLvdsMappingLane2Fmt0EW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane2_fmt_0_f(
        &mut self,
    ) -> CfgLvdsMappingLane2Fmt0FW<CfgLvdsMappingLane2Fmt0Spec> {
        CfgLvdsMappingLane2Fmt0FW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane2_fmt_0_g(
        &mut self,
    ) -> CfgLvdsMappingLane2Fmt0GW<CfgLvdsMappingLane2Fmt0Spec> {
        CfgLvdsMappingLane2Fmt0GW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Please refer to LVDS Mapping Format section for confiuration details"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_mapping_lane2_fmt_0_h(
        &mut self,
    ) -> CfgLvdsMappingLane2Fmt0HW<CfgLvdsMappingLane2Fmt0Spec> {
        CfgLvdsMappingLane2Fmt0HW::new(self, 28)
    }
}
#[doc = "CFG_LVDS_MAPPING_LANE2_FMT_0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_mapping_lane2_fmt_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_mapping_lane2_fmt_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgLvdsMappingLane2Fmt0Spec;
impl crate::RegisterSpec for CfgLvdsMappingLane2Fmt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_lvds_mapping_lane2_fmt_0::R`](R) reader structure"]
impl crate::Readable for CfgLvdsMappingLane2Fmt0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_lvds_mapping_lane2_fmt_0::W`](W) writer structure"]
impl crate::Writable for CfgLvdsMappingLane2Fmt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_LVDS_MAPPING_LANE2_FMT_0 to value 0"]
impl crate::Resettable for CfgLvdsMappingLane2Fmt0Spec {
    const RESET_VALUE: u32 = 0;
}
