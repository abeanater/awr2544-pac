#[doc = "Register `CLR_CBUFF_ECC_REG` reader"]
pub type R = crate::R<ClrCbuffEccRegSpec>;
#[doc = "Register `CLR_CBUFF_ECC_REG` writer"]
pub type W = crate::W<ClrCbuffEccRegSpec>;
#[doc = "Field `ceccadd` reader - 0:0\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
pub type CeccaddR = crate::BitReader;
#[doc = "Field `ceccadd` writer - 0:0\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
pub type CeccaddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ceccsbe` reader - 8:8\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
pub type CeccsbeR = crate::BitReader;
#[doc = "Field `ceccsbe` writer - 8:8\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
pub type CeccsbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ceccdbe` reader - 9:9\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
pub type CeccdbeR = crate::BitReader;
#[doc = "Field `ceccdbe` writer - 9:9\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
pub type CeccdbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn ceccadd(&self) -> CeccaddR {
        CeccaddR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn ceccsbe(&self) -> CeccsbeR {
        CeccsbeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn ceccdbe(&self) -> CeccdbeR {
        CeccdbeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn ceccadd(&mut self) -> CeccaddW<ClrCbuffEccRegSpec> {
        CeccaddW::new(self, 0)
    }
    #[doc = "Bits 1:7"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<ClrCbuffEccRegSpec> {
        Nu1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn ceccsbe(&mut self) -> CeccsbeW<ClrCbuffEccRegSpec> {
        CeccsbeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Register field corresponding to STAT_CBUFF_ECC. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn ceccdbe(&mut self) -> CeccdbeW<ClrCbuffEccRegSpec> {
        CeccdbeW::new(self, 9)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<ClrCbuffEccRegSpec> {
        Nu2W::new(self, 10)
    }
}
#[doc = "CLR_CBUFF_ECC_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_cbuff_ecc_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_cbuff_ecc_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrCbuffEccRegSpec;
impl crate::RegisterSpec for ClrCbuffEccRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_cbuff_ecc_reg::R`](R) reader structure"]
impl crate::Readable for ClrCbuffEccRegSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_cbuff_ecc_reg::W`](W) writer structure"]
impl crate::Writable for ClrCbuffEccRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR_CBUFF_ECC_REG to value 0"]
impl crate::Resettable for ClrCbuffEccRegSpec {
    const RESET_VALUE: u32 = 0;
}
