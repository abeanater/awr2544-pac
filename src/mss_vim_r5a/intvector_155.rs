#[doc = "Register `INTVECTOR_155` reader"]
pub type R = crate::R<Intvector155Spec>;
#[doc = "Register `INTVECTOR_155` writer"]
pub type W = crate::W<Intvector155Spec>;
#[doc = "Field `RES20` reader - 1:0\\]
Reserved. Read as 0. The lower 2 bits of the 32-bit vector address are always 0. Vector addresses must be 32-bit aligned."]
pub type Res20R = crate::FieldReader;
#[doc = "Field `RES20` writer - 1:0\\]
Reserved. Read as 0. The lower 2 bits of the 32-bit vector address are always 0. Vector addresses must be 32-bit aligned."]
pub type Res20W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDR` reader - 31:2\\]
This is the 32-bit Vector Address associated with interrupt Q. It is the address that will be reflected in the IRQ Vector Address (Base Address + 0x18) or FIQ Vector Address (Base Address + 0x1C) and the VECADDR pin when interrupt Q is the active interrupt. Internally, these values are kept in a RAM. The FIQ and IRQ state machines have priority access to this RAM. Writes to this register will be piped internally, but further writes to the MMR interface may be stalled until this write has a chance to complete in the RAM. The new Vector Address will not take effect until this write completes to the RAM. In order to tell if this write has completed, software may read this register back. That read will not be able to complete unless the write has landed. Reads to this register will stall the MMR interface until the read is able to be completed at the RAM."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:2\\]
This is the 32-bit Vector Address associated with interrupt Q. It is the address that will be reflected in the IRQ Vector Address (Base Address + 0x18) or FIQ Vector Address (Base Address + 0x1C) and the VECADDR pin when interrupt Q is the active interrupt. Internally, these values are kept in a RAM. The FIQ and IRQ state machines have priority access to this RAM. Writes to this register will be piped internally, but further writes to the MMR interface may be stalled until this write has a chance to complete in the RAM. The new Vector Address will not take effect until this write completes to the RAM. In order to tell if this write has completed, software may read this register back. That read will not be able to complete unless the write has landed. Reads to this register will stall the MMR interface until the read is able to be completed at the RAM."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved. Read as 0. The lower 2 bits of the 32-bit vector address are always 0. Vector addresses must be 32-bit aligned."]
    #[inline(always)]
    pub fn res20(&self) -> Res20R {
        Res20R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
This is the 32-bit Vector Address associated with interrupt Q. It is the address that will be reflected in the IRQ Vector Address (Base Address + 0x18) or FIQ Vector Address (Base Address + 0x1C) and the VECADDR pin when interrupt Q is the active interrupt. Internally, these values are kept in a RAM. The FIQ and IRQ state machines have priority access to this RAM. Writes to this register will be piped internally, but further writes to the MMR interface may be stalled until this write has a chance to complete in the RAM. The new Vector Address will not take effect until this write completes to the RAM. In order to tell if this write has completed, software may read this register back. That read will not be able to complete unless the write has landed. Reads to this register will stall the MMR interface until the read is able to be completed at the RAM."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved. Read as 0. The lower 2 bits of the 32-bit vector address are always 0. Vector addresses must be 32-bit aligned."]
    #[inline(always)]
    #[must_use]
    pub fn res20(&mut self) -> Res20W<Intvector155Spec> {
        Res20W::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
This is the 32-bit Vector Address associated with interrupt Q. It is the address that will be reflected in the IRQ Vector Address (Base Address + 0x18) or FIQ Vector Address (Base Address + 0x1C) and the VECADDR pin when interrupt Q is the active interrupt. Internally, these values are kept in a RAM. The FIQ and IRQ state machines have priority access to this RAM. Writes to this register will be piped internally, but further writes to the MMR interface may be stalled until this write has a chance to complete in the RAM. The new Vector Address will not take effect until this write completes to the RAM. In order to tell if this write has completed, software may read this register back. That read will not be able to complete unless the write has landed. Reads to this register will stall the MMR interface until the read is able to be completed at the RAM."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Intvector155Spec> {
        AddrW::new(self, 2)
    }
}
#[doc = "Interrupt Q Vector Register (Q is 0 to 255 , Q= M+1 x 32) h2000 + Q x h159\n\nYou can [`read`](crate::Reg::read) this register and get [`intvector_155::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvector_155::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intvector155Spec;
impl crate::RegisterSpec for Intvector155Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intvector_155::R`](R) reader structure"]
impl crate::Readable for Intvector155Spec {}
#[doc = "`write(|w| ..)` method takes [`intvector_155::W`](W) writer structure"]
impl crate::Writable for Intvector155Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTVECTOR_155 to value 0"]
impl crate::Resettable for Intvector155Spec {
    const RESET_VALUE: u32 = 0;
}
