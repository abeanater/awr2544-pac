#[doc = "Register `RS232_SLEEP_CLK_DIV` reader"]
pub type R = crate::R<Rs232SleepClkDivSpec>;
#[doc = "Register `RS232_SLEEP_CLK_DIV` writer"]
pub type W = crate::W<Rs232SleepClkDivSpec>;
#[doc = "Field `div_val` reader - 31:0\\]
The Divider value for RS232 sleep clock generation from RCclk"]
pub type DivValR = crate::FieldReader<u32>;
#[doc = "Field `div_val` writer - 31:0\\]
The Divider value for RS232 sleep clock generation from RCclk"]
pub type DivValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The Divider value for RS232 sleep clock generation from RCclk"]
    #[inline(always)]
    pub fn div_val(&self) -> DivValR {
        DivValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The Divider value for RS232 sleep clock generation from RCclk"]
    #[inline(always)]
    #[must_use]
    pub fn div_val(&mut self) -> DivValW<Rs232SleepClkDivSpec> {
        DivValW::new(self, 0)
    }
}
#[doc = "RS232_SLEEP_CLK_DIV\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_sleep_clk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_sleep_clk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs232SleepClkDivSpec;
impl crate::RegisterSpec for Rs232SleepClkDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs232_sleep_clk_div::R`](R) reader structure"]
impl crate::Readable for Rs232SleepClkDivSpec {}
#[doc = "`write(|w| ..)` method takes [`rs232_sleep_clk_div::W`](W) writer structure"]
impl crate::Writable for Rs232SleepClkDivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RS232_SLEEP_CLK_DIV to value 0"]
impl crate::Resettable for Rs232SleepClkDivSpec {
    const RESET_VALUE: u32 = 0;
}
