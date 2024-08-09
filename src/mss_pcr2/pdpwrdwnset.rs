#[doc = "Register `PDPWRDWNSET` reader"]
pub type R = crate::R<PdpwrdwnsetSpec>;
#[doc = "Register `PDPWRDWNSET` writer"]
pub type W = crate::W<PdpwrdwnsetSpec>;
#[doc = "Field `PD_PWRDWN_SET` reader - 0:0\\]
Readable in both user and privileged modes. 1 = Clock to the debug frame needs to be powered down. 0 = Clock to the debug frame needs to be powered up. Writable only in privileged mode 1 = Bit 0 when written 1, will get set in both PDPWRDWNSET and PDPWRDWNCLR registers. The other bits are not affected. 0 = Has no effect"]
pub type PdPwrdwnSetR = crate::BitReader;
#[doc = "Field `PD_PWRDWN_SET` writer - 0:0\\]
Readable in both user and privileged modes. 1 = Clock to the debug frame needs to be powered down. 0 = Clock to the debug frame needs to be powered up. Writable only in privileged mode 1 = Bit 0 when written 1, will get set in both PDPWRDWNSET and PDPWRDWNCLR registers. The other bits are not affected. 0 = Has no effect"]
pub type PdPwrdwnSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Readable in both user and privileged modes. 1 = Clock to the debug frame needs to be powered down. 0 = Clock to the debug frame needs to be powered up. Writable only in privileged mode 1 = Bit 0 when written 1, will get set in both PDPWRDWNSET and PDPWRDWNCLR registers. The other bits are not affected. 0 = Has no effect"]
    #[inline(always)]
    pub fn pd_pwrdwn_set(&self) -> PdPwrdwnSetR {
        PdPwrdwnSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Readable in both user and privileged modes. 1 = Clock to the debug frame needs to be powered down. 0 = Clock to the debug frame needs to be powered up. Writable only in privileged mode 1 = Bit 0 when written 1, will get set in both PDPWRDWNSET and PDPWRDWNCLR registers. The other bits are not affected. 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pd_pwrdwn_set(&mut self) -> PdPwrdwnSetW<PdpwrdwnsetSpec> {
        PdPwrdwnSetW::new(self, 0)
    }
}
#[doc = "Set-only register to powerdown the debug frame\n\nYou can [`read`](crate::Reg::read) this register and get [`pdpwrdwnset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdpwrdwnset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdpwrdwnsetSpec;
impl crate::RegisterSpec for PdpwrdwnsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdpwrdwnset::R`](R) reader structure"]
impl crate::Readable for PdpwrdwnsetSpec {}
#[doc = "`write(|w| ..)` method takes [`pdpwrdwnset::W`](W) writer structure"]
impl crate::Writable for PdpwrdwnsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDPWRDWNSET to value 0"]
impl crate::Resettable for PdpwrdwnsetSpec {
    const RESET_VALUE: u32 = 0;
}
