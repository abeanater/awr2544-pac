#[doc = "Register `MSS_L2_BANKB_PD_STATUS` reader"]
pub type R = crate::R<MssL2BankbPdStatusSpec>;
#[doc = "Register `MSS_L2_BANKB_PD_STATUS` writer"]
pub type W = crate::W<MssL2BankbPdStatusSpec>;
#[doc = "Field `aonout` reader - 0:0\\]
SW status indicating the 'ponin' of MSS_L2_BANKB"]
pub type AonoutR = crate::BitReader;
#[doc = "Field `aonout` writer - 0:0\\]
SW status indicating the 'ponin' of MSS_L2_BANKB"]
pub type AonoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `agoodout` reader - 1:1\\]
SW status indicating the 'pgoodin' of MSS_L2_BANKB"]
pub type AgoodoutR = crate::BitReader;
#[doc = "Field `agoodout` writer - 1:1\\]
SW status indicating the 'pgoodin' of MSS_L2_BANKB"]
pub type AgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SW status indicating the 'ponin' of MSS_L2_BANKB"]
    #[inline(always)]
    pub fn aonout(&self) -> AonoutR {
        AonoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SW status indicating the 'pgoodin' of MSS_L2_BANKB"]
    #[inline(always)]
    pub fn agoodout(&self) -> AgoodoutR {
        AgoodoutR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SW status indicating the 'ponin' of MSS_L2_BANKB"]
    #[inline(always)]
    #[must_use]
    pub fn aonout(&mut self) -> AonoutW<MssL2BankbPdStatusSpec> {
        AonoutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SW status indicating the 'pgoodin' of MSS_L2_BANKB"]
    #[inline(always)]
    #[must_use]
    pub fn agoodout(&mut self) -> AgoodoutW<MssL2BankbPdStatusSpec> {
        AgoodoutW::new(self, 1)
    }
}
#[doc = "MSS_L2_BANKB_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_bankb_pd_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_bankb_pd_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssL2BankbPdStatusSpec;
impl crate::RegisterSpec for MssL2BankbPdStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_l2_bankb_pd_status::R`](R) reader structure"]
impl crate::Readable for MssL2BankbPdStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_l2_bankb_pd_status::W`](W) writer structure"]
impl crate::Writable for MssL2BankbPdStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_L2_BANKB_PD_STATUS to value 0"]
impl crate::Resettable for MssL2BankbPdStatusSpec {
    const RESET_VALUE: u32 = 0;
}
