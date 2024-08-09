#[doc = "Register `DBG_ACK_CTL1` reader"]
pub type R = crate::R<DbgAckCtl1Spec>;
#[doc = "Register `DBG_ACK_CTL1` writer"]
pub type W = crate::W<DbgAckCtl1Spec>;
#[doc = "Field `DSS_MCRC` reader - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DssMcrcR = crate::FieldReader;
#[doc = "Field `DSS_MCRC` writer - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DssMcrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSS_HWA` reader - 30:28\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DssHwaR = crate::FieldReader;
#[doc = "Field `DSS_HWA` writer - 30:28\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type DssHwaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 24:26 - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn dss_mcrc(&self) -> DssMcrcR {
        DssMcrcR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn dss_hwa(&self) -> DssHwaR {
        DssHwaR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - 26:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn dss_mcrc(&mut self) -> DssMcrcW<DbgAckCtl1Spec> {
        DssMcrcW::new(self, 24)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn dss_hwa(&mut self) -> DssHwaW<DbgAckCtl1Spec> {
        DssHwaW::new(self, 28)
    }
}
#[doc = "DBG_ACK_CTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_ack_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_ack_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgAckCtl1Spec;
impl crate::RegisterSpec for DbgAckCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_ack_ctl1::R`](R) reader structure"]
impl crate::Readable for DbgAckCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_ack_ctl1::W`](W) writer structure"]
impl crate::Writable for DbgAckCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_ACK_CTL1 to value 0"]
impl crate::Resettable for DbgAckCtl1Spec {
    const RESET_VALUE: u32 = 0;
}
