#[doc = "Register `DSS_EDMA_RST_CTRL` reader"]
pub type R = crate::R<DssEdmaRstCtrlSpec>;
#[doc = "Register `DSS_EDMA_RST_CTRL` writer"]
pub type W = crate::W<DssEdmaRstCtrlSpec>;
#[doc = "Field `assert_tc0` reader - 2:0\\]
This register is for Debug Purposes only. Reset control for DSS TPTCA0 Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type AssertTc0R = crate::FieldReader;
#[doc = "Field `assert_tc0` writer - 2:0\\]
This register is for Debug Purposes only. Reset control for DSS TPTCA0 Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type AssertTc0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `assert_tc1` reader - 6:4\\]
This register is for Debug Purposes only. Reset control for DSS TPTCA1 Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type AssertTc1R = crate::FieldReader;
#[doc = "Field `assert_tc1` writer - 6:4\\]
This register is for Debug Purposes only. Reset control for DSS TPTCA1 Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type AssertTc1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This register is for Debug Purposes only. Reset control for DSS TPTCA0 Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    pub fn assert_tc0(&self) -> AssertTc0R {
        AssertTc0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
This register is for Debug Purposes only. Reset control for DSS TPTCA1 Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    pub fn assert_tc1(&self) -> AssertTc1R {
        AssertTc1R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This register is for Debug Purposes only. Reset control for DSS TPTCA0 Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    #[must_use]
    pub fn assert_tc0(&mut self) -> AssertTc0W<DssEdmaRstCtrlSpec> {
        AssertTc0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
This register is for Debug Purposes only. Reset control for DSS TPTCA1 Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    #[must_use]
    pub fn assert_tc1(&mut self) -> AssertTc1W<DssEdmaRstCtrlSpec> {
        AssertTc1W::new(self, 4)
    }
}
#[doc = "DSS_EDMA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_edma_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_edma_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssEdmaRstCtrlSpec;
impl crate::RegisterSpec for DssEdmaRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_edma_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for DssEdmaRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_edma_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for DssEdmaRstCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_EDMA_RST_CTRL to value 0"]
impl crate::Resettable for DssEdmaRstCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
