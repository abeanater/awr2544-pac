#[doc = "Register `HSM_RTI_CLK_STATUS` reader"]
pub type R = crate::R<HsmRtiClkStatusSpec>;
#[doc = "Register `HSM_RTI_CLK_STATUS` writer"]
pub type W = crate::W<HsmRtiClkStatusSpec>;
#[doc = "Field `clkinuse` reader - 7:0\\]
Status shows the source clock slected for HSM_RTI"]
pub type ClkinuseR = crate::FieldReader;
#[doc = "Field `clkinuse` writer - 7:0\\]
Status shows the source clock slected for HSM_RTI"]
pub type ClkinuseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `currdivider` reader - 15:8\\]
Status shows the current divider value choosen for HSM_RTI"]
pub type CurrdividerR = crate::FieldReader;
#[doc = "Field `currdivider` writer - 15:8\\]
Status shows the current divider value choosen for HSM_RTI"]
pub type CurrdividerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for HSM_RTI"]
    #[inline(always)]
    pub fn clkinuse(&self) -> ClkinuseR {
        ClkinuseR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Status shows the current divider value choosen for HSM_RTI"]
    #[inline(always)]
    pub fn currdivider(&self) -> CurrdividerR {
        CurrdividerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for HSM_RTI"]
    #[inline(always)]
    #[must_use]
    pub fn clkinuse(&mut self) -> ClkinuseW<HsmRtiClkStatusSpec> {
        ClkinuseW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Status shows the current divider value choosen for HSM_RTI"]
    #[inline(always)]
    #[must_use]
    pub fn currdivider(&mut self) -> CurrdividerW<HsmRtiClkStatusSpec> {
        CurrdividerW::new(self, 8)
    }
}
#[doc = "HSM_RTI_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_rti_clk_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_rti_clk_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsmRtiClkStatusSpec;
impl crate::RegisterSpec for HsmRtiClkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsm_rti_clk_status::R`](R) reader structure"]
impl crate::Readable for HsmRtiClkStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hsm_rti_clk_status::W`](W) writer structure"]
impl crate::Writable for HsmRtiClkStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSM_RTI_CLK_STATUS to value 0"]
impl crate::Resettable for HsmRtiClkStatusSpec {
    const RESET_VALUE: u32 = 0;
}
