#[doc = "Register `DBG_ACK_CTL0` reader"]
pub type R = crate::R<DbgAckCtl0Spec>;
#[doc = "Register `DBG_ACK_CTL0` writer"]
pub type W = crate::W<DbgAckCtl0Spec>;
#[doc = "Field `DSS_DCCA` reader - 2:0\\]
RESERVED: Dont Use"]
pub type DssDccaR = crate::FieldReader;
#[doc = "Field `DSS_DCCA` writer - 2:0\\]
RESERVED: Dont Use"]
pub type DssDccaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSS_DCCB` reader - 6:4\\]
RESERVED: Dont Use"]
pub type DssDccbR = crate::FieldReader;
#[doc = "Field `DSS_DCCB` writer - 6:4\\]
RESERVED: Dont Use"]
pub type DssDccbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSS_RTIA` reader - 10:8\\]
ERESERVED: Dont Use"]
pub type DssRtiaR = crate::FieldReader;
#[doc = "Field `DSS_RTIA` writer - 10:8\\]
ERESERVED: Dont Use"]
pub type DssRtiaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSS_RTIB` reader - 14:12\\]
RESERVED: Dont Use"]
pub type DssRtibR = crate::FieldReader;
#[doc = "Field `DSS_RTIB` writer - 14:12\\]
RESERVED: Dont Use"]
pub type DssRtibW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSS_SCIA` reader - 18:16\\]
RESERVED: Dont Use"]
pub type DssSciaR = crate::FieldReader;
#[doc = "Field `DSS_SCIA` writer - 18:16\\]
RESERVED: Dont Use"]
pub type DssSciaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSS_WDT` reader - 22:20\\]
RESERVED: Dont Use"]
pub type DssWdtR = crate::FieldReader;
#[doc = "Field `DSS_WDT` writer - 22:20\\]
RESERVED: Dont Use"]
pub type DssWdtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn dss_dcca(&self) -> DssDccaR {
        DssDccaR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn dss_dccb(&self) -> DssDccbR {
        DssDccbR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
ERESERVED: Dont Use"]
    #[inline(always)]
    pub fn dss_rtia(&self) -> DssRtiaR {
        DssRtiaR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn dss_rtib(&self) -> DssRtibR {
        DssRtibR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn dss_scia(&self) -> DssSciaR {
        DssSciaR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn dss_wdt(&self) -> DssWdtR {
        DssWdtR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn dss_dcca(&mut self) -> DssDccaW<DbgAckCtl0Spec> {
        DssDccaW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn dss_dccb(&mut self) -> DssDccbW<DbgAckCtl0Spec> {
        DssDccbW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
ERESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn dss_rtia(&mut self) -> DssRtiaW<DbgAckCtl0Spec> {
        DssRtiaW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn dss_rtib(&mut self) -> DssRtibW<DbgAckCtl0Spec> {
        DssRtibW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn dss_scia(&mut self) -> DssSciaW<DbgAckCtl0Spec> {
        DssSciaW::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn dss_wdt(&mut self) -> DssWdtW<DbgAckCtl0Spec> {
        DssWdtW::new(self, 20)
    }
}
#[doc = "DBG_ACK_CTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_ack_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_ack_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgAckCtl0Spec;
impl crate::RegisterSpec for DbgAckCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_ack_ctl0::R`](R) reader structure"]
impl crate::Readable for DbgAckCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_ack_ctl0::W`](W) writer structure"]
impl crate::Writable for DbgAckCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_ACK_CTL0 to value 0"]
impl crate::Resettable for DbgAckCtl0Spec {
    const RESET_VALUE: u32 = 0;
}
