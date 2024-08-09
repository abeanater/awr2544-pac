#[doc = "Register `MEM_ACCESS_ERR_STATUS` reader"]
pub type R = crate::R<MemAccessErrStatusSpec>;
#[doc = "Register `MEM_ACCESS_ERR_STATUS` writer"]
pub type W = crate::W<MemAccessErrStatusSpec>;
#[doc = "Field `dmem0` reader - 0:0\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem0 at the same time"]
pub type Dmem0R = crate::BitReader;
#[doc = "Field `dmem0` writer - 0:0\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem0 at the same time"]
pub type Dmem0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem1` reader - 1:1\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem1 at the same time"]
pub type Dmem1R = crate::BitReader;
#[doc = "Field `dmem1` writer - 1:1\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem1 at the same time"]
pub type Dmem1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem2` reader - 2:2\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem2 at the same time"]
pub type Dmem2R = crate::BitReader;
#[doc = "Field `dmem2` writer - 2:2\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem2 at the same time"]
pub type Dmem2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem3` reader - 3:3\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem3 at the same time"]
pub type Dmem3R = crate::BitReader;
#[doc = "Field `dmem3` writer - 3:3\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem3 at the same time"]
pub type Dmem3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem0 at the same time"]
    #[inline(always)]
    pub fn dmem0(&self) -> Dmem0R {
        Dmem0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem1 at the same time"]
    #[inline(always)]
    pub fn dmem1(&self) -> Dmem1R {
        Dmem1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem2 at the same time"]
    #[inline(always)]
    pub fn dmem2(&self) -> Dmem2R {
        Dmem2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem3 at the same time"]
    #[inline(always)]
    pub fn dmem3(&self) -> Dmem3R {
        Dmem3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem0 at the same time"]
    #[inline(always)]
    #[must_use]
    pub fn dmem0(&mut self) -> Dmem0W<MemAccessErrStatusSpec> {
        Dmem0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem1 at the same time"]
    #[inline(always)]
    #[must_use]
    pub fn dmem1(&mut self) -> Dmem1W<MemAccessErrStatusSpec> {
        Dmem1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem2 at the same time"]
    #[inline(always)]
    #[must_use]
    pub fn dmem2(&mut self) -> Dmem2W<MemAccessErrStatusSpec> {
        Dmem2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates if more than 1 master ( DMA,CM4,Accelerator) are trying to access the dmem3 at the same time"]
    #[inline(always)]
    #[must_use]
    pub fn dmem3(&mut self) -> Dmem3W<MemAccessErrStatusSpec> {
        Dmem3W::new(self, 3)
    }
}
#[doc = "MEM_ACCESS_ERR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_access_err_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_access_err_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemAccessErrStatusSpec;
impl crate::RegisterSpec for MemAccessErrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_access_err_status::R`](R) reader structure"]
impl crate::Readable for MemAccessErrStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_access_err_status::W`](W) writer structure"]
impl crate::Writable for MemAccessErrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ACCESS_ERR_STATUS to value 0"]
impl crate::Resettable for MemAccessErrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
