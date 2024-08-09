#[doc = "Register `IRQVEC` reader"]
pub type R = crate::R<IrqvecSpec>;
#[doc = "Register `IRQVEC` writer"]
pub type W = crate::W<IrqvecSpec>;
#[doc = "Field `RES21` reader - 1:0\\]
RESERVE FIELD"]
pub type Res21R = crate::FieldReader;
#[doc = "Field `RES21` writer - 1:0\\]
RESERVE FIELD"]
pub type Res21W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDR` reader - 31:2\\]
Upper 30 bits of the 32-bit vector address. Only valid if the Prioritized IRQ Register valid flag is true."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:2\\]
Upper 30 bits of the 32-bit vector address. Only valid if the Prioritized IRQ Register valid flag is true."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res21(&self) -> Res21R {
        Res21R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Upper 30 bits of the 32-bit vector address. Only valid if the Prioritized IRQ Register valid flag is true."]
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
    pub fn res21(&mut self) -> Res21W<IrqvecSpec> {
        Res21W::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Upper 30 bits of the 32-bit vector address. Only valid if the Prioritized IRQ Register valid flag is true."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<IrqvecSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "The IRQ Vector Address Register contains the 32-bit address of the interrupt vector for the current pendind IRQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`irqvec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqvec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqvecSpec;
impl crate::RegisterSpec for IrqvecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqvec::R`](R) reader structure"]
impl crate::Readable for IrqvecSpec {}
#[doc = "`write(|w| ..)` method takes [`irqvec::W`](W) writer structure"]
impl crate::Writable for IrqvecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQVEC to value 0"]
impl crate::Resettable for IrqvecSpec {
    const RESET_VALUE: u32 = 0;
}
