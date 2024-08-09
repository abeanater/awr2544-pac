#[doc = "Register `RSS_DBG_ACK_CTL0` reader"]
pub type R = crate::R<RssDbgAckCtl0Spec>;
#[doc = "Register `RSS_DBG_ACK_CTL0` writer"]
pub type W = crate::W<RssDbgAckCtl0Spec>;
#[doc = "Field `frc` reader - 2:0\\]
emulation suspend signal control . Writing '111' would ungate the emulation suspend signal to the FRC"]
pub type FrcR = crate::FieldReader;
#[doc = "Field `frc` writer - 2:0\\]
emulation suspend signal control . Writing '111' would ungate the emulation suspend signal to the FRC"]
pub type FrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
emulation suspend signal control . Writing '111' would ungate the emulation suspend signal to the FRC"]
    #[inline(always)]
    pub fn frc(&self) -> FrcR {
        FrcR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
emulation suspend signal control . Writing '111' would ungate the emulation suspend signal to the FRC"]
    #[inline(always)]
    #[must_use]
    pub fn frc(&mut self) -> FrcW<RssDbgAckCtl0Spec> {
        FrcW::new(self, 0)
    }
}
#[doc = "RSS_DBG_ACK_CTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_dbg_ack_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_dbg_ack_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssDbgAckCtl0Spec;
impl crate::RegisterSpec for RssDbgAckCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_dbg_ack_ctl0::R`](R) reader structure"]
impl crate::Readable for RssDbgAckCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rss_dbg_ack_ctl0::W`](W) writer structure"]
impl crate::Writable for RssDbgAckCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_DBG_ACK_CTL0 to value 0"]
impl crate::Resettable for RssDbgAckCtl0Spec {
    const RESET_VALUE: u32 = 0;
}
