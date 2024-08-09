#[doc = "Register `ALE_ALE_OAM_LB_CTRL` reader"]
pub type R = crate::R<AleAleOamLbCtrlSpec>;
#[doc = "Register `ALE_ALE_OAM_LB_CTRL` writer"]
pub type W = crate::W<AleAleOamLbCtrlSpec>;
#[doc = "Field `THE_ IOAM_LB_CTRL_ALLOWS` reader - 1:0\\]
The ~ioam_lb_ctrl allows any port to be put into OAM loopback, that is any packet received will be returned to the same port with an egressop of 0xFF which swaps the source and destination address. BPDUs will still flow through as normal so that OAM can be remotly requested and disabled."]
pub type TheIoamLbCtrlAllowsR = crate::FieldReader;
#[doc = "Field `THE_ IOAM_LB_CTRL_ALLOWS` writer - 1:0\\]
The ~ioam_lb_ctrl allows any port to be put into OAM loopback, that is any packet received will be returned to the same port with an egressop of 0xFF which swaps the source and destination address. BPDUs will still flow through as normal so that OAM can be remotly requested and disabled."]
pub type TheIoamLbCtrlAllowsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
The ~ioam_lb_ctrl allows any port to be put into OAM loopback, that is any packet received will be returned to the same port with an egressop of 0xFF which swaps the source and destination address. BPDUs will still flow through as normal so that OAM can be remotly requested and disabled."]
    #[inline(always)]
    pub fn the_ioam_lb_ctrl_allows(&self) -> TheIoamLbCtrlAllowsR {
        TheIoamLbCtrlAllowsR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
The ~ioam_lb_ctrl allows any port to be put into OAM loopback, that is any packet received will be returned to the same port with an egressop of 0xFF which swaps the source and destination address. BPDUs will still flow through as normal so that OAM can be remotly requested and disabled."]
    #[inline(always)]
    #[must_use]
    pub fn the_ioam_lb_ctrl_allows(&mut self) -> TheIoamLbCtrlAllowsW<AleAleOamLbCtrlSpec> {
        TheIoamLbCtrlAllowsW::new(self, 0)
    }
}
#[doc = "The ALE OAM Control allows ports to be put into OAM Loopback, only non-supervisor packet are looped back to the source port.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_oam_lb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_oam_lb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleOamLbCtrlSpec;
impl crate::RegisterSpec for AleAleOamLbCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_oam_lb_ctrl::R`](R) reader structure"]
impl crate::Readable for AleAleOamLbCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_oam_lb_ctrl::W`](W) writer structure"]
impl crate::Writable for AleAleOamLbCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_OAM_LB_CTRL to value 0"]
impl crate::Resettable for AleAleOamLbCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
