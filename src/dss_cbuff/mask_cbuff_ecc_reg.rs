#[doc = "Register `MASK_CBUFF_ECC_REG` reader"]
pub type R = crate::R<MaskCbuffEccRegSpec>;
#[doc = "Register `MASK_CBUFF_ECC_REG` writer"]
pub type W = crate::W<MaskCbuffEccRegSpec>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `meccsbe` reader - 8:8\\]
0 : Single bit error indications are unmasked 1 : Single bit error indications are Masked"]
pub type MeccsbeR = crate::BitReader;
#[doc = "Field `meccsbe` writer - 8:8\\]
0 : Single bit error indications are unmasked 1 : Single bit error indications are Masked"]
pub type MeccsbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `meccdbe` reader - 9:9\\]
0 : Double bit error indications are unmasked 1 : Double bit error indications are Masked"]
pub type MeccdbeR = crate::BitReader;
#[doc = "Field `meccdbe` writer - 9:9\\]
0 : Double bit error indications are unmasked 1 : Double bit error indications are Masked"]
pub type MeccdbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
0 : Single bit error indications are unmasked 1 : Single bit error indications are Masked"]
    #[inline(always)]
    pub fn meccsbe(&self) -> MeccsbeR {
        MeccsbeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0 : Double bit error indications are unmasked 1 : Double bit error indications are Masked"]
    #[inline(always)]
    pub fn meccdbe(&self) -> MeccdbeR {
        MeccdbeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<MaskCbuffEccRegSpec> {
        Nu1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0 : Single bit error indications are unmasked 1 : Single bit error indications are Masked"]
    #[inline(always)]
    #[must_use]
    pub fn meccsbe(&mut self) -> MeccsbeW<MaskCbuffEccRegSpec> {
        MeccsbeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
0 : Double bit error indications are unmasked 1 : Double bit error indications are Masked"]
    #[inline(always)]
    #[must_use]
    pub fn meccdbe(&mut self) -> MeccdbeW<MaskCbuffEccRegSpec> {
        MeccdbeW::new(self, 9)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<MaskCbuffEccRegSpec> {
        Nu2W::new(self, 10)
    }
}
#[doc = "MASK_CBUFF_ECC_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_cbuff_ecc_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_cbuff_ecc_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskCbuffEccRegSpec;
impl crate::RegisterSpec for MaskCbuffEccRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_cbuff_ecc_reg::R`](R) reader structure"]
impl crate::Readable for MaskCbuffEccRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mask_cbuff_ecc_reg::W`](W) writer structure"]
impl crate::Writable for MaskCbuffEccRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK_CBUFF_ECC_REG to value 0"]
impl crate::Resettable for MaskCbuffEccRegSpec {
    const RESET_VALUE: u32 = 0;
}
