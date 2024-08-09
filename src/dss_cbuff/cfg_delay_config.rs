#[doc = "Register `CFG_DELAY_CONFIG` reader"]
pub type R = crate::R<CfgDelayConfigSpec>;
#[doc = "Register `CFG_DELAY_CONFIG` writer"]
pub type W = crate::W<CfgDelayConfigSpec>;
#[doc = "Field `CFG_SPHDR_DELAY` reader - 7:0\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Short packet. This is a Debug feature. Not requred in Programming model"]
pub type CfgSphdrDelayR = crate::FieldReader;
#[doc = "Field `CFG_SPHDR_DELAY` writer - 7:0\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Short packet. This is a Debug feature. Not requred in Programming model"]
pub type CfgSphdrDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFG_LPHDR_DELAY` reader - 15:8\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Long packet Header. This is a Debug feature. Not requred in Programming model"]
pub type CfgLphdrDelayR = crate::FieldReader;
#[doc = "Field `CFG_LPHDR_DELAY` writer - 15:8\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Long packet Header. This is a Debug feature. Not requred in Programming model"]
pub type CfgLphdrDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFG_DATA_WR_DELAY` reader - 23:16\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Long packet Payload. This is a Debug feature. Not requred in Programming model"]
pub type CfgDataWrDelayR = crate::FieldReader;
#[doc = "Field `CFG_DATA_WR_DELAY` writer - 23:16\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Long packet Payload. This is a Debug feature. Not requred in Programming model"]
pub type CfgDataWrDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Short packet. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn cfg_sphdr_delay(&self) -> CfgSphdrDelayR {
        CfgSphdrDelayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Long packet Header. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn cfg_lphdr_delay(&self) -> CfgLphdrDelayR {
        CfgLphdrDelayR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Long packet Payload. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn cfg_data_wr_delay(&self) -> CfgDataWrDelayR {
        CfgDataWrDelayR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Short packet. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_sphdr_delay(&mut self) -> CfgSphdrDelayW<CfgDelayConfigSpec> {
        CfgSphdrDelayW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Long packet Header. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lphdr_delay(&mut self) -> CfgLphdrDelayW<CfgDelayConfigSpec> {
        CfgLphdrDelayW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Feature CSI2 only Programming : Configure an additional delay after sending a Long packet Payload. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_data_wr_delay(&mut self) -> CfgDataWrDelayW<CfgDelayConfigSpec> {
        CfgDataWrDelayW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<CfgDelayConfigSpec> {
        NuW::new(self, 24)
    }
}
#[doc = "Delay Config Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_delay_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_delay_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDelayConfigSpec;
impl crate::RegisterSpec for CfgDelayConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_delay_config::R`](R) reader structure"]
impl crate::Readable for CfgDelayConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_delay_config::W`](W) writer structure"]
impl crate::Writable for CfgDelayConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DELAY_CONFIG to value 0"]
impl crate::Resettable for CfgDelayConfigSpec {
    const RESET_VALUE: u32 = 0;
}
