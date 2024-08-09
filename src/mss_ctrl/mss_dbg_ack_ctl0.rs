#[doc = "Register `MSS_DBG_ACK_CTL0` reader"]
pub type R = crate::R<MssDbgAckCtl0Spec>;
#[doc = "Register `MSS_DBG_ACK_CTL0` writer"]
pub type W = crate::W<MssDbgAckCtl0Spec>;
#[doc = "Field `ccca` reader - 2:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type CccaR = crate::FieldReader;
#[doc = "Field `ccca` writer - 2:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type CccaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cccb` reader - 6:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type CccbR = crate::FieldReader;
#[doc = "Field `cccb` writer - 6:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type CccbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcca` reader - 10:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DccaR = crate::FieldReader;
#[doc = "Field `dcca` writer - 10:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DccaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dccb` reader - 14:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DccbR = crate::FieldReader;
#[doc = "Field `dccb` writer - 14:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DccbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dccc` reader - 18:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DcccR = crate::FieldReader;
#[doc = "Field `dccc` writer - 18:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DcccW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dccd` reader - 22:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DccdR = crate::FieldReader;
#[doc = "Field `dccd` writer - 22:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DccdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cpsw` reader - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type CpswR = crate::FieldReader;
#[doc = "Field `cpsw` writer - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type CpswW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn ccca(&self) -> CccaR {
        CccaR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn cccb(&self) -> CccbR {
        CccbR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn dcca(&self) -> DccaR {
        DccaR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn dccb(&self) -> DccbR {
        DccbR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn dccc(&self) -> DcccR {
        DcccR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn dccd(&self) -> DccdR {
        DccdR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn cpsw(&self) -> CpswR {
        CpswR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn ccca(&mut self) -> CccaW<MssDbgAckCtl0Spec> {
        CccaW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn cccb(&mut self) -> CccbW<MssDbgAckCtl0Spec> {
        CccbW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn dcca(&mut self) -> DccaW<MssDbgAckCtl0Spec> {
        DccaW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn dccb(&mut self) -> DccbW<MssDbgAckCtl0Spec> {
        DccbW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn dccc(&mut self) -> DcccW<MssDbgAckCtl0Spec> {
        DcccW::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn dccd(&mut self) -> DccdW<MssDbgAckCtl0Spec> {
        DccdW::new(self, 20)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn cpsw(&mut self) -> CpswW<MssDbgAckCtl0Spec> {
        CpswW::new(self, 24)
    }
}
#[doc = "MSS_DBG_ACK_CTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dbg_ack_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dbg_ack_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDbgAckCtl0Spec;
impl crate::RegisterSpec for MssDbgAckCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dbg_ack_ctl0::R`](R) reader structure"]
impl crate::Readable for MssDbgAckCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mss_dbg_ack_ctl0::W`](W) writer structure"]
impl crate::Writable for MssDbgAckCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DBG_ACK_CTL0 to value 0"]
impl crate::Resettable for MssDbgAckCtl0Spec {
    const RESET_VALUE: u32 = 0;
}
