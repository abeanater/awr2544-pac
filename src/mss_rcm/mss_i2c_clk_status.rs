#[doc = "Register `MSS_I2C_CLK_STATUS` reader"]
pub type R = crate::R<MssI2cClkStatusSpec>;
#[doc = "Register `MSS_I2C_CLK_STATUS` writer"]
pub type W = crate::W<MssI2cClkStatusSpec>;
#[doc = "Field `clkinuse` reader - 7:0\\]
Status shows the source clock slected for I2C"]
pub type ClkinuseR = crate::FieldReader;
#[doc = "Field `clkinuse` writer - 7:0\\]
Status shows the source clock slected for I2C"]
pub type ClkinuseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `currdivider` reader - 15:8\\]
Status shows the current divider value choosen for I2C"]
pub type CurrdividerR = crate::FieldReader;
#[doc = "Field `currdivider` writer - 15:8\\]
Status shows the current divider value choosen for I2C"]
pub type CurrdividerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for I2C"]
    #[inline(always)]
    pub fn clkinuse(&self) -> ClkinuseR {
        ClkinuseR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Status shows the current divider value choosen for I2C"]
    #[inline(always)]
    pub fn currdivider(&self) -> CurrdividerR {
        CurrdividerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for I2C"]
    #[inline(always)]
    #[must_use]
    pub fn clkinuse(&mut self) -> ClkinuseW<MssI2cClkStatusSpec> {
        ClkinuseW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Status shows the current divider value choosen for I2C"]
    #[inline(always)]
    #[must_use]
    pub fn currdivider(&mut self) -> CurrdividerW<MssI2cClkStatusSpec> {
        CurrdividerW::new(self, 8)
    }
}
#[doc = "MSS_I2C_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_i2c_clk_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_i2c_clk_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssI2cClkStatusSpec;
impl crate::RegisterSpec for MssI2cClkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_i2c_clk_status::R`](R) reader structure"]
impl crate::Readable for MssI2cClkStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_i2c_clk_status::W`](W) writer structure"]
impl crate::Writable for MssI2cClkStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_I2C_CLK_STATUS to value 0"]
impl crate::Resettable for MssI2cClkStatusSpec {
    const RESET_VALUE: u32 = 0;
}
