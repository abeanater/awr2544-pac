#[doc = "Register `ICOUNT3` reader"]
pub type R = crate::R<Icount3Spec>;
#[doc = "Register `ICOUNT3` writer"]
pub type W = crate::W<Icount3Spec>;
#[doc = "Field `COUNT` reader - 15:0\\]
Actual number of remaining DMA transfer COUNTx is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set. Since the real COUNTx is always ICLOUNTx +1, the 17th bit of COUNTx is available on DMAxCTRL(6) bit."]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - 15:0\\]
Actual number of remaining DMA transfer COUNTx is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set. Since the real COUNTx is always ICLOUNTx +1, the 17th bit of COUNTx is available on DMAxCTRL(6) bit."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ICOUNT` reader - 31:16\\]
Initial Number of DMA transfers. ICOUNTx is used to preset the transfer counter COUNTx. Every time COUNTx hits zero it is reloaded with ICOUNTx. The real number of transfer equals ICOUNTx plus one. If ONESHOTx is set, ICOUNTx defines the number of DMA transfers that are performed before the MibSPI automatically disables the DMA channels. If NOBRKx is set, ICOUNTx defines the number of DMA transfers that are performed in one sequence without a transfer from any other buffer"]
pub type IcountR = crate::FieldReader<u16>;
#[doc = "Field `ICOUNT` writer - 31:16\\]
Initial Number of DMA transfers. ICOUNTx is used to preset the transfer counter COUNTx. Every time COUNTx hits zero it is reloaded with ICOUNTx. The real number of transfer equals ICOUNTx plus one. If ONESHOTx is set, ICOUNTx defines the number of DMA transfers that are performed before the MibSPI automatically disables the DMA channels. If NOBRKx is set, ICOUNTx defines the number of DMA transfers that are performed in one sequence without a transfer from any other buffer"]
pub type IcountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Actual number of remaining DMA transfer COUNTx is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set. Since the real COUNTx is always ICLOUNTx +1, the 17th bit of COUNTx is available on DMAxCTRL(6) bit."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Initial Number of DMA transfers. ICOUNTx is used to preset the transfer counter COUNTx. Every time COUNTx hits zero it is reloaded with ICOUNTx. The real number of transfer equals ICOUNTx plus one. If ONESHOTx is set, ICOUNTx defines the number of DMA transfers that are performed before the MibSPI automatically disables the DMA channels. If NOBRKx is set, ICOUNTx defines the number of DMA transfers that are performed in one sequence without a transfer from any other buffer"]
    #[inline(always)]
    pub fn icount(&self) -> IcountR {
        IcountR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Actual number of remaining DMA transfer COUNTx is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set. Since the real COUNTx is always ICLOUNTx +1, the 17th bit of COUNTx is available on DMAxCTRL(6) bit."]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<Icount3Spec> {
        CountW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Initial Number of DMA transfers. ICOUNTx is used to preset the transfer counter COUNTx. Every time COUNTx hits zero it is reloaded with ICOUNTx. The real number of transfer equals ICOUNTx plus one. If ONESHOTx is set, ICOUNTx defines the number of DMA transfers that are performed before the MibSPI automatically disables the DMA channels. If NOBRKx is set, ICOUNTx defines the number of DMA transfers that are performed in one sequence without a transfer from any other buffer"]
    #[inline(always)]
    #[must_use]
    pub fn icount(&mut self) -> IcountW<Icount3Spec> {
        IcountW::new(self, 16)
    }
}
#[doc = "MibSPI DMAxCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`icount3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icount3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icount3Spec;
impl crate::RegisterSpec for Icount3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icount3::R`](R) reader structure"]
impl crate::Readable for Icount3Spec {}
#[doc = "`write(|w| ..)` method takes [`icount3::W`](W) writer structure"]
impl crate::Writable for Icount3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICOUNT3 to value 0"]
impl crate::Resettable for Icount3Spec {
    const RESET_VALUE: u32 = 0;
}
