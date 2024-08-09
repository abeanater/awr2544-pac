#[doc = "Register `STAT_CBUFF_ECC_REG` reader"]
pub type R = crate::R<StatCbuffEccRegSpec>;
#[doc = "Register `STAT_CBUFF_ECC_REG` writer"]
pub type W = crate::W<StatCbuffEccRegSpec>;
#[doc = "Field `seccadd` reader - 5:0\\]
6-bit address where the ECC error occurred. It is valid when either seccsbe or seccdbe is set. If none of them is set, then the addr does not mean anything."]
pub type SeccaddR = crate::FieldReader;
#[doc = "Field `seccadd` writer - 5:0\\]
6-bit address where the ECC error occurred. It is valid when either seccsbe or seccdbe is set. If none of them is set, then the addr does not mean anything."]
pub type SeccaddW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `seccsbe` reader - 8:8\\]
0 : No Single bit error 1 : Indicates a single bit error has occurred"]
pub type SeccsbeR = crate::BitReader;
#[doc = "Field `seccsbe` writer - 8:8\\]
0 : No Single bit error 1 : Indicates a single bit error has occurred"]
pub type SeccsbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `seccdbe` reader - 9:9\\]
0 : No Double bit error 1 : Indicates a double bit error has occurred"]
pub type SeccdbeR = crate::BitReader;
#[doc = "Field `seccdbe` writer - 9:9\\]
0 : No Double bit error 1 : Indicates a double bit error has occurred"]
pub type SeccdbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
6-bit address where the ECC error occurred. It is valid when either seccsbe or seccdbe is set. If none of them is set, then the addr does not mean anything."]
    #[inline(always)]
    pub fn seccadd(&self) -> SeccaddR {
        SeccaddR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
0 : No Single bit error 1 : Indicates a single bit error has occurred"]
    #[inline(always)]
    pub fn seccsbe(&self) -> SeccsbeR {
        SeccsbeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0 : No Double bit error 1 : Indicates a double bit error has occurred"]
    #[inline(always)]
    pub fn seccdbe(&self) -> SeccdbeR {
        SeccdbeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
6-bit address where the ECC error occurred. It is valid when either seccsbe or seccdbe is set. If none of them is set, then the addr does not mean anything."]
    #[inline(always)]
    #[must_use]
    pub fn seccadd(&mut self) -> SeccaddW<StatCbuffEccRegSpec> {
        SeccaddW::new(self, 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<StatCbuffEccRegSpec> {
        Nu1W::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
0 : No Single bit error 1 : Indicates a single bit error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn seccsbe(&mut self) -> SeccsbeW<StatCbuffEccRegSpec> {
        SeccsbeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
0 : No Double bit error 1 : Indicates a double bit error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn seccdbe(&mut self) -> SeccdbeW<StatCbuffEccRegSpec> {
        SeccdbeW::new(self, 9)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<StatCbuffEccRegSpec> {
        Nu2W::new(self, 10)
    }
}
#[doc = "STAT_CBUFF_ECC_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cbuff_ecc_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_cbuff_ecc_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatCbuffEccRegSpec;
impl crate::RegisterSpec for StatCbuffEccRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_cbuff_ecc_reg::R`](R) reader structure"]
impl crate::Readable for StatCbuffEccRegSpec {}
#[doc = "`write(|w| ..)` method takes [`stat_cbuff_ecc_reg::W`](W) writer structure"]
impl crate::Writable for StatCbuffEccRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT_CBUFF_ECC_REG to value 0"]
impl crate::Resettable for StatCbuffEccRegSpec {
    const RESET_VALUE: u32 = 0;
}
