#[doc = "Register `PDPWRDWNCLR` reader"]
pub type R = crate::R<PdpwrdwnclrSpec>;
#[doc = "Register `PDPWRDWNCLR` writer"]
pub type W = crate::W<PdpwrdwnclrSpec>;
#[doc = "Field `PD_PWRDWN_CLR` reader - 0:0\\]
Readable in both user and privileged modes. 1 = The clock to the debug frame needs to be powered down. 0 = The clock to the debug frame needs to be powered up. Writable only in privileged mode 1 = Bit 0 when written 1, will get cleared in both PDPWRDWNSET and PDPWRDWNCLR registers. The other bits are not affected. 0 = Has no effect"]
pub type PdPwrdwnClrR = crate::BitReader;
#[doc = "Field `PD_PWRDWN_CLR` writer - 0:0\\]
Readable in both user and privileged modes. 1 = The clock to the debug frame needs to be powered down. 0 = The clock to the debug frame needs to be powered up. Writable only in privileged mode 1 = Bit 0 when written 1, will get cleared in both PDPWRDWNSET and PDPWRDWNCLR registers. The other bits are not affected. 0 = Has no effect"]
pub type PdPwrdwnClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Readable in both user and privileged modes. 1 = The clock to the debug frame needs to be powered down. 0 = The clock to the debug frame needs to be powered up. Writable only in privileged mode 1 = Bit 0 when written 1, will get cleared in both PDPWRDWNSET and PDPWRDWNCLR registers. The other bits are not affected. 0 = Has no effect"]
    #[inline(always)]
    pub fn pd_pwrdwn_clr(&self) -> PdPwrdwnClrR {
        PdPwrdwnClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Readable in both user and privileged modes. 1 = The clock to the debug frame needs to be powered down. 0 = The clock to the debug frame needs to be powered up. Writable only in privileged mode 1 = Bit 0 when written 1, will get cleared in both PDPWRDWNSET and PDPWRDWNCLR registers. The other bits are not affected. 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pd_pwrdwn_clr(&mut self) -> PdPwrdwnClrW<PdpwrdwnclrSpec> {
        PdPwrdwnClrW::new(self, 0)
    }
}
#[doc = "Clear-only register to deassert the debug frameΓÇÖs powerdown bit\n\nYou can [`read`](crate::Reg::read) this register and get [`pdpwrdwnclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdpwrdwnclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdpwrdwnclrSpec;
impl crate::RegisterSpec for PdpwrdwnclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdpwrdwnclr::R`](R) reader structure"]
impl crate::Readable for PdpwrdwnclrSpec {}
#[doc = "`write(|w| ..)` method takes [`pdpwrdwnclr::W`](W) writer structure"]
impl crate::Writable for PdpwrdwnclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDPWRDWNCLR to value 0"]
impl crate::Resettable for PdpwrdwnclrSpec {
    const RESET_VALUE: u32 = 0;
}
