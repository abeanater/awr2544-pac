#[doc = "Register `ANA_REG_CLK_CTRL_REG2_CLKTOP` reader"]
pub type R = crate::R<AnaRegClkCtrlReg2ClktopSpec>;
#[doc = "Register `ANA_REG_CLK_CTRL_REG2_CLKTOP` writer"]
pub type W = crate::W<AnaRegClkCtrlReg2ClktopSpec>;
#[doc = "Field `RESERVED0` reader - 31:0\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 31:0\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegClkCtrlReg2ClktopSpec> {
        Reserved0W::new(self, 0)
    }
}
#[doc = "ANA_REG_CLK_CTRL_REG2_CLKTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_ctrl_reg2_clktop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_ctrl_reg2_clktop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegClkCtrlReg2ClktopSpec;
impl crate::RegisterSpec for AnaRegClkCtrlReg2ClktopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_clk_ctrl_reg2_clktop::R`](R) reader structure"]
impl crate::Readable for AnaRegClkCtrlReg2ClktopSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_clk_ctrl_reg2_clktop::W`](W) writer structure"]
impl crate::Writable for AnaRegClkCtrlReg2ClktopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_CLK_CTRL_REG2_CLKTOP to value 0"]
impl crate::Resettable for AnaRegClkCtrlReg2ClktopSpec {
    const RESET_VALUE: u32 = 0;
}
