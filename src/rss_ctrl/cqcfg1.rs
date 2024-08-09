#[doc = "Register `CQCFG1` reader"]
pub type R = crate::R<Cqcfg1Spec>;
#[doc = "Register `CQCFG1` writer"]
pub type W = crate::W<Cqcfg1Spec>;
#[doc = "Field `CQDATAWIDTH` reader - 1:0\\]
This is used to appropriately pack the valid CQ data bits in appropriate bits in the CQ memory. 00, 01->Raw 16, 10-->Raw 12, 11-->Raw14"]
pub type CqdatawidthR = crate::FieldReader;
#[doc = "Field `CQDATAWIDTH` writer - 1:0\\]
This is used to appropriately pack the valid CQ data bits in appropriate bits in the CQ memory. 00, 01->Raw 16, 10-->Raw 12, 11-->Raw14"]
pub type CqdatawidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CQ96BITPACKEN` reader - 3:3\\]
This is used to pack the CQ data into only the LSB 96 bits of each row of the CQ memory. This can be used in 3 channel mode of LVDS where the ADC data and Chirp Params occupy only LSB 96 bits of each memory row."]
pub type Cq96bitpackenR = crate::BitReader;
#[doc = "Field `CQ96BITPACKEN` writer - 3:3\\]
This is used to pack the CQ data into only the LSB 96 bits of each row of the CQ memory. This can be used in 3 channel mode of LVDS where the ADC data and Chirp Params occupy only LSB 96 bits of each memory row."]
pub type Cq96bitpackenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQ0BASEADDR` reader - 12:4\\]
128-bit Address offset which inidcates the start address for storing CQ0 (Wide Band Energy detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
pub type Cq0baseaddrR = crate::FieldReader<u16>;
#[doc = "Field `CQ0BASEADDR` writer - 12:4\\]
128-bit Address offset which inidcates the start address for storing CQ0 (Wide Band Energy detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
pub type Cq0baseaddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CQ1BASEADDR` reader - 21:13\\]
128-bit Address offset which inidcates the start address for storing CQ0 (Signal Image Band Energy detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
pub type Cq1baseaddrR = crate::FieldReader<u16>;
#[doc = "Field `CQ1BASEADDR` writer - 21:13\\]
128-bit Address offset which inidcates the start address for storing CQ0 (Signal Image Band Energy detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
pub type Cq1baseaddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CQ2BASEADDR` reader - 30:22\\]
128-bit Address offset which inidcates the start address for storing CQ0 (ADC/RxIF Saturation Detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
pub type Cq2baseaddrR = crate::FieldReader<u16>;
#[doc = "Field `CQ2BASEADDR` writer - 30:22\\]
128-bit Address offset which inidcates the start address for storing CQ0 (ADC/RxIF Saturation Detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
pub type Cq2baseaddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
This is used to appropriately pack the valid CQ data bits in appropriate bits in the CQ memory. 00, 01->Raw 16, 10-->Raw 12, 11-->Raw14"]
    #[inline(always)]
    pub fn cqdatawidth(&self) -> CqdatawidthR {
        CqdatawidthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
This is used to pack the CQ data into only the LSB 96 bits of each row of the CQ memory. This can be used in 3 channel mode of LVDS where the ADC data and Chirp Params occupy only LSB 96 bits of each memory row."]
    #[inline(always)]
    pub fn cq96bitpacken(&self) -> Cq96bitpackenR {
        Cq96bitpackenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:12 - 12:4\\]
128-bit Address offset which inidcates the start address for storing CQ0 (Wide Band Energy detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
    #[inline(always)]
    pub fn cq0baseaddr(&self) -> Cq0baseaddrR {
        Cq0baseaddrR::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 13:21 - 21:13\\]
128-bit Address offset which inidcates the start address for storing CQ0 (Signal Image Band Energy detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
    #[inline(always)]
    pub fn cq1baseaddr(&self) -> Cq1baseaddrR {
        Cq1baseaddrR::new(((self.bits >> 13) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:30 - 30:22\\]
128-bit Address offset which inidcates the start address for storing CQ0 (ADC/RxIF Saturation Detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
    #[inline(always)]
    pub fn cq2baseaddr(&self) -> Cq2baseaddrR {
        Cq2baseaddrR::new(((self.bits >> 22) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
This is used to appropriately pack the valid CQ data bits in appropriate bits in the CQ memory. 00, 01->Raw 16, 10-->Raw 12, 11-->Raw14"]
    #[inline(always)]
    #[must_use]
    pub fn cqdatawidth(&mut self) -> CqdatawidthW<Cqcfg1Spec> {
        CqdatawidthW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This is used to pack the CQ data into only the LSB 96 bits of each row of the CQ memory. This can be used in 3 channel mode of LVDS where the ADC data and Chirp Params occupy only LSB 96 bits of each memory row."]
    #[inline(always)]
    #[must_use]
    pub fn cq96bitpacken(&mut self) -> Cq96bitpackenW<Cqcfg1Spec> {
        Cq96bitpackenW::new(self, 3)
    }
    #[doc = "Bits 4:12 - 12:4\\]
128-bit Address offset which inidcates the start address for storing CQ0 (Wide Band Energy detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
    #[inline(always)]
    #[must_use]
    pub fn cq0baseaddr(&mut self) -> Cq0baseaddrW<Cqcfg1Spec> {
        Cq0baseaddrW::new(self, 4)
    }
    #[doc = "Bits 13:21 - 21:13\\]
128-bit Address offset which inidcates the start address for storing CQ0 (Signal Image Band Energy detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
    #[inline(always)]
    #[must_use]
    pub fn cq1baseaddr(&mut self) -> Cq1baseaddrW<Cqcfg1Spec> {
        Cq1baseaddrW::new(self, 13)
    }
    #[doc = "Bits 22:30 - 30:22\\]
128-bit Address offset which inidcates the start address for storing CQ0 (ADC/RxIF Saturation Detection) from the start of CQ memory. This is not the byte address offset but 128 bit address offset"]
    #[inline(always)]
    #[must_use]
    pub fn cq2baseaddr(&mut self) -> Cq2baseaddrW<Cqcfg1Spec> {
        Cq2baseaddrW::new(self, 22)
    }
}
#[doc = "CQCFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cqcfg1Spec;
impl crate::RegisterSpec for Cqcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcfg1::R`](R) reader structure"]
impl crate::Readable for Cqcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cqcfg1::W`](W) writer structure"]
impl crate::Writable for Cqcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQCFG1 to value 0"]
impl crate::Resettable for Cqcfg1Spec {
    const RESET_VALUE: u32 = 0;
}
