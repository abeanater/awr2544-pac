#[doc = "Register `DEDVEC` reader"]
pub type R = crate::R<DedvecSpec>;
#[doc = "Register `DEDVEC` writer"]
pub type W = crate::W<DedvecSpec>;
#[doc = "Field `RES23` reader - 1:0\\]
RESERVE FIELD"]
pub type Res23R = crate::FieldReader;
#[doc = "Field `RES23` writer - 1:0\\]
RESERVE FIELD"]
pub type Res23W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDR` reader - 31:2\\]
Upper 30 bits of the 32-bit vector address."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:2\\]
Upper 30 bits of the 32-bit vector address."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res23(&self) -> Res23R {
        Res23R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Upper 30 bits of the 32-bit vector address."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res23(&mut self) -> Res23W<DedvecSpec> {
        Res23W::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Upper 30 bits of the 32-bit vector address."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<DedvecSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "The DED Vector Address contains a default vector address for when an uncorrectable error is detected for an active IRQ or FIQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`dedvec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dedvec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedvecSpec;
impl crate::RegisterSpec for DedvecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dedvec::R`](R) reader structure"]
impl crate::Readable for DedvecSpec {}
#[doc = "`write(|w| ..)` method takes [`dedvec::W`](W) writer structure"]
impl crate::Writable for DedvecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEDVEC to value 0"]
impl crate::Resettable for DedvecSpec {
    const RESET_VALUE: u32 = 0;
}
