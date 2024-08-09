#[doc = "Register `RS232_BITINTERVAL` reader"]
pub type R = crate::R<Rs232BitintervalSpec>;
#[doc = "Register `RS232_BITINTERVAL` writer"]
pub type W = crate::W<Rs232BitintervalSpec>;
#[doc = "Field `bitinterval` reader - 31:0\\]
RS232 Bit Interval. 10 bit clock interval is selceted based on the value of MSS_CR5_CLK_SRC_SEL \\[9:0\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x0 \\[19:10\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x1 \\[29:20\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x2"]
pub type BitintervalR = crate::FieldReader<u32>;
#[doc = "Field `bitinterval` writer - 31:0\\]
RS232 Bit Interval. 10 bit clock interval is selceted based on the value of MSS_CR5_CLK_SRC_SEL \\[9:0\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x0 \\[19:10\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x1 \\[29:20\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x2"]
pub type BitintervalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
RS232 Bit Interval. 10 bit clock interval is selceted based on the value of MSS_CR5_CLK_SRC_SEL \\[9:0\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x0 \\[19:10\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x1 \\[29:20\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x2"]
    #[inline(always)]
    pub fn bitinterval(&self) -> BitintervalR {
        BitintervalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
RS232 Bit Interval. 10 bit clock interval is selceted based on the value of MSS_CR5_CLK_SRC_SEL \\[9:0\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x0 \\[19:10\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x1 \\[29:20\\]
used as RS232 Bit inteval when MSS_CR5_CLK_SRC_SEL = 0x2"]
    #[inline(always)]
    #[must_use]
    pub fn bitinterval(&mut self) -> BitintervalW<Rs232BitintervalSpec> {
        BitintervalW::new(self, 0)
    }
}
#[doc = "RS232_BITINTERVAL\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_bitinterval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_bitinterval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs232BitintervalSpec;
impl crate::RegisterSpec for Rs232BitintervalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs232_bitinterval::R`](R) reader structure"]
impl crate::Readable for Rs232BitintervalSpec {}
#[doc = "`write(|w| ..)` method takes [`rs232_bitinterval::W`](W) writer structure"]
impl crate::Writable for Rs232BitintervalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RS232_BITINTERVAL to value 0"]
impl crate::Resettable for Rs232BitintervalSpec {
    const RESET_VALUE: u32 = 0;
}
