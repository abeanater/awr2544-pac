#[doc = "Register `MSS_I2C_CLK_GATE` reader"]
pub type R = crate::R<MssI2cClkGateSpec>;
#[doc = "Register `MSS_I2C_CLK_GATE` writer"]
pub type W = crate::W<MssI2cClkGateSpec>;
#[doc = "Field `gated` reader - 2:0\\]
writing '111' will gate clock for I2C"]
pub type GatedR = crate::FieldReader;
#[doc = "Field `gated` writer - 2:0\\]
writing '111' will gate clock for I2C"]
pub type GatedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' will gate clock for I2C"]
    #[inline(always)]
    pub fn gated(&self) -> GatedR {
        GatedR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' will gate clock for I2C"]
    #[inline(always)]
    #[must_use]
    pub fn gated(&mut self) -> GatedW<MssI2cClkGateSpec> {
        GatedW::new(self, 0)
    }
}
#[doc = "MSS_I2C_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_i2c_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_i2c_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssI2cClkGateSpec;
impl crate::RegisterSpec for MssI2cClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_i2c_clk_gate::R`](R) reader structure"]
impl crate::Readable for MssI2cClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_i2c_clk_gate::W`](W) writer structure"]
impl crate::Writable for MssI2cClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_I2C_CLK_GATE to value 0"]
impl crate::Resettable for MssI2cClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
