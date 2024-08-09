#[doc = "Register `MSS_DBG_ACK_CTL1` reader"]
pub type R = crate::R<MssDbgAckCtl1Spec>;
#[doc = "Register `MSS_DBG_ACK_CTL1` writer"]
pub type W = crate::W<MssDbgAckCtl1Spec>;
#[doc = "Field `dcan` reader - 2:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DcanR = crate::FieldReader;
#[doc = "Field `dcan` writer - 2:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DcanW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rti` reader - 6:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type RtiR = crate::FieldReader;
#[doc = "Field `rti` writer - 6:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type RtiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `wdt` reader - 10:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type WdtR = crate::FieldReader;
#[doc = "Field `wdt` writer - 10:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type WdtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `mcrc` reader - 14:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type McrcR = crate::FieldReader;
#[doc = "Field `mcrc` writer - 14:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type McrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `i2c` reader - 18:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type I2cR = crate::FieldReader;
#[doc = "Field `i2c` writer - 18:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type I2cW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `scia` reader - 22:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type SciaR = crate::FieldReader;
#[doc = "Field `scia` writer - 22:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type SciaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `scib` reader - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type ScibR = crate::FieldReader;
#[doc = "Field `scib` writer - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type ScibW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn dcan(&self) -> DcanR {
        DcanR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn rti(&self) -> RtiR {
        RtiR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn mcrc(&self) -> McrcR {
        McrcR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn i2c(&self) -> I2cR {
        I2cR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn scia(&self) -> SciaR {
        SciaR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn scib(&self) -> ScibR {
        ScibR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn dcan(&mut self) -> DcanW<MssDbgAckCtl1Spec> {
        DcanW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn rti(&mut self) -> RtiW<MssDbgAckCtl1Spec> {
        RtiW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WdtW<MssDbgAckCtl1Spec> {
        WdtW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn mcrc(&mut self) -> McrcW<MssDbgAckCtl1Spec> {
        McrcW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2cW<MssDbgAckCtl1Spec> {
        I2cW::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn scia(&mut self) -> SciaW<MssDbgAckCtl1Spec> {
        SciaW::new(self, 20)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn scib(&mut self) -> ScibW<MssDbgAckCtl1Spec> {
        ScibW::new(self, 24)
    }
}
#[doc = "MSS_DBG_ACK_CTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dbg_ack_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dbg_ack_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDbgAckCtl1Spec;
impl crate::RegisterSpec for MssDbgAckCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dbg_ack_ctl1::R`](R) reader structure"]
impl crate::Readable for MssDbgAckCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mss_dbg_ack_ctl1::W`](W) writer structure"]
impl crate::Writable for MssDbgAckCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DBG_ACK_CTL1 to value 0"]
impl crate::Resettable for MssDbgAckCtl1Spec {
    const RESET_VALUE: u32 = 0;
}
