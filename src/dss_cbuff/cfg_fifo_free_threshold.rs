#[doc = "Register `CFG_FIFO_FREE_THRESHOLD` reader"]
pub type R = crate::R<CfgFifoFreeThresholdSpec>;
#[doc = "Register `CFG_FIFO_FREE_THRESHOLD` writer"]
pub type W = crate::W<CfgFifoFreeThresholdSpec>;
#[doc = "Field `CFG_FIFO_FREE_THRESHOLD0` reader - 7:0\\]
CSI2 only Programming : Configure the threshold used to fill the FIFO0 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register"]
pub type CfgFifoFreeThreshold0R = crate::FieldReader;
#[doc = "Field `CFG_FIFO_FREE_THRESHOLD0` writer - 7:0\\]
CSI2 only Programming : Configure the threshold used to fill the FIFO0 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register"]
pub type CfgFifoFreeThreshold0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFG_FIFO_FREE_THRESHOLD1` reader - 15:8\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO1 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
pub type CfgFifoFreeThreshold1R = crate::FieldReader;
#[doc = "Field `CFG_FIFO_FREE_THRESHOLD1` writer - 15:8\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO1 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
pub type CfgFifoFreeThreshold1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFG_FIFO_FREE_THRESHOLD2` reader - 23:16\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO2 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
pub type CfgFifoFreeThreshold2R = crate::FieldReader;
#[doc = "Field `CFG_FIFO_FREE_THRESHOLD2` writer - 23:16\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO2 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
pub type CfgFifoFreeThreshold2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFG_FIFO_FREE_THRESHOLD3` reader - 31:24\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO3 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
pub type CfgFifoFreeThreshold3R = crate::FieldReader;
#[doc = "Field `CFG_FIFO_FREE_THRESHOLD3` writer - 31:24\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO3 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
pub type CfgFifoFreeThreshold3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
CSI2 only Programming : Configure the threshold used to fill the FIFO0 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register"]
    #[inline(always)]
    pub fn cfg_fifo_free_threshold0(&self) -> CfgFifoFreeThreshold0R {
        CfgFifoFreeThreshold0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO1 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
    #[inline(always)]
    pub fn cfg_fifo_free_threshold1(&self) -> CfgFifoFreeThreshold1R {
        CfgFifoFreeThreshold1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO2 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
    #[inline(always)]
    pub fn cfg_fifo_free_threshold2(&self) -> CfgFifoFreeThreshold2R {
        CfgFifoFreeThreshold2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO3 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
    #[inline(always)]
    pub fn cfg_fifo_free_threshold3(&self) -> CfgFifoFreeThreshold3R {
        CfgFifoFreeThreshold3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
CSI2 only Programming : Configure the threshold used to fill the FIFO0 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_fifo_free_threshold0(&mut self) -> CfgFifoFreeThreshold0W<CfgFifoFreeThresholdSpec> {
        CfgFifoFreeThreshold0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO1 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_fifo_free_threshold1(&mut self) -> CfgFifoFreeThreshold1W<CfgFifoFreeThresholdSpec> {
        CfgFifoFreeThreshold1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO2 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_fifo_free_threshold2(&mut self) -> CfgFifoFreeThreshold2W<CfgFifoFreeThresholdSpec> {
        CfgFifoFreeThreshold2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI Internal Feature CSI2 only Programming : Configure the threshold used to fill the FIFO3 in the CSI Protocol engine. CBUFF will send data to the Protocol Engine only if there is a larger number of Free slots that that configured in this register. By default, only 1 FIFO will be used so register programming is not Required in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_fifo_free_threshold3(&mut self) -> CfgFifoFreeThreshold3W<CfgFifoFreeThresholdSpec> {
        CfgFifoFreeThreshold3W::new(self, 24)
    }
}
#[doc = "CSI2 FIFO threshold for transferring data from CBUFF to CSI2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_fifo_free_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_fifo_free_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgFifoFreeThresholdSpec;
impl crate::RegisterSpec for CfgFifoFreeThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_fifo_free_threshold::R`](R) reader structure"]
impl crate::Readable for CfgFifoFreeThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_fifo_free_threshold::W`](W) writer structure"]
impl crate::Writable for CfgFifoFreeThresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_FIFO_FREE_THRESHOLD to value 0"]
impl crate::Resettable for CfgFifoFreeThresholdSpec {
    const RESET_VALUE: u32 = 0;
}
