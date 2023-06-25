# constraints
# ad9361 (SWAP == 0x1)

set_property  -dict {PACKAGE_PIN  U18  IOSTANDARD LVCMOS18 } [get_ports rx_clk_in]
set_property  -dict {PACKAGE_PIN  Y16  IOSTANDARD LVCMOS18 } [get_ports rx_frame_in]
set_property  -dict {PACKAGE_PIN  V13  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[0]]
set_property  -dict {PACKAGE_PIN  T14  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[1]]
set_property  -dict {PACKAGE_PIN  U13  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[2]]
set_property  -dict {PACKAGE_PIN  U12  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[3]]
set_property  -dict {PACKAGE_PIN  T12  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[4]]
set_property  -dict {PACKAGE_PIN  T10  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[5]]
set_property  -dict {PACKAGE_PIN  T11  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[6]]
set_property  -dict {PACKAGE_PIN  T15  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[7]]
set_property  -dict {PACKAGE_PIN  V12  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[8]]
set_property  -dict {PACKAGE_PIN  W13  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[9]]
set_property  -dict {PACKAGE_PIN  P14  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[10]]
set_property  -dict {PACKAGE_PIN  R14  IOSTANDARD LVCMOS18 } [get_ports rx_data_in[11]]

set_property  -dict {PACKAGE_PIN  P15  IOSTANDARD LVCMOS18} [get_ports tx_clk_out]
set_property  -dict {PACKAGE_PIN  T16  IOSTANDARD LVCMOS18} [get_ports tx_frame_out]
set_property  -dict {PACKAGE_PIN  T20  IOSTANDARD LVCMOS18} [get_ports tx_data_out[0]]
set_property  -dict {PACKAGE_PIN  Y19  IOSTANDARD LVCMOS18} [get_ports tx_data_out[1]]
set_property  -dict {PACKAGE_PIN  U20  IOSTANDARD LVCMOS18} [get_ports tx_data_out[2]]
set_property  -dict {PACKAGE_PIN  V20  IOSTANDARD LVCMOS18} [get_ports tx_data_out[3]]
set_property  -dict {PACKAGE_PIN  W20  IOSTANDARD LVCMOS18} [get_ports tx_data_out[4]]
set_property  -dict {PACKAGE_PIN  Y18  IOSTANDARD LVCMOS18} [get_ports tx_data_out[5]]
set_property  -dict {PACKAGE_PIN  W16  IOSTANDARD LVCMOS18} [get_ports tx_data_out[6]]
set_property  -dict {PACKAGE_PIN  V16  IOSTANDARD LVCMOS18} [get_ports tx_data_out[7]]
set_property  -dict {PACKAGE_PIN  R17   IOSTANDARD LVCMOS18} [get_ports tx_data_out[8]]
set_property  -dict {PACKAGE_PIN  R16   IOSTANDARD LVCMOS18} [get_ports tx_data_out[9]]
set_property  -dict {PACKAGE_PIN  R18   IOSTANDARD LVCMOS18} [get_ports tx_data_out[10]]
set_property  -dict {PACKAGE_PIN  T17   IOSTANDARD LVCMOS18} [get_ports tx_data_out[11]]

set_property  -dict {PACKAGE_PIN  W14  IOSTANDARD LVCMOS18} [get_ports gpio_status[0]]
set_property  -dict {PACKAGE_PIN  Y14  IOSTANDARD LVCMOS18} [get_ports gpio_status[1]]
set_property  -dict {PACKAGE_PIN  N18  IOSTANDARD LVCMOS18} [get_ports gpio_status[2]]
set_property  -dict {PACKAGE_PIN  P19  IOSTANDARD LVCMOS18} [get_ports gpio_status[3]]
set_property  -dict {PACKAGE_PIN  V17  IOSTANDARD LVCMOS18} [get_ports gpio_status[4]]
set_property  -dict {PACKAGE_PIN  V18  IOSTANDARD LVCMOS18} [get_ports gpio_status[5]]
set_property  -dict {PACKAGE_PIN  W18  IOSTANDARD LVCMOS18} [get_ports gpio_status[6]]
set_property  -dict {PACKAGE_PIN  W19  IOSTANDARD LVCMOS18} [get_ports gpio_status[7]]

set_property  -dict {PACKAGE_PIN  E17  IOSTANDARD LVCMOS18} [get_ports gpio_ctl[0]]
set_property  -dict {PACKAGE_PIN  D18  IOSTANDARD LVCMOS18} [get_ports gpio_ctl[1]]
set_property  -dict {PACKAGE_PIN  E18  IOSTANDARD LVCMOS18} [get_ports gpio_ctl[2]]
set_property  -dict {PACKAGE_PIN  E19  IOSTANDARD LVCMOS18} [get_ports gpio_ctl[3]]

set_property  -dict {PACKAGE_PIN  W15  IOSTANDARD LVCMOS18} [get_ports gpio_en_agc]
set_property  -dict {PACKAGE_PIN  P18  IOSTANDARD LVCMOS18} [get_ports gpio_resetb]

set_property  -dict {PACKAGE_PIN  U15  IOSTANDARD LVCMOS18} [get_ports enable]
set_property  -dict {PACKAGE_PIN  U14  IOSTANDARD LVCMOS18} [get_ports txnrx]

set_property  -dict {PACKAGE_PIN  A20  IOSTANDARD LVCMOS18  PULLTYPE PULLUP} [get_ports spi_csn]
set_property  -dict {PACKAGE_PIN  B19  IOSTANDARD LVCMOS18} [get_ports spi_clk]
set_property  -dict {PACKAGE_PIN  B20  IOSTANDARD LVCMOS18} [get_ports spi_mosi]
set_property  -dict {PACKAGE_PIN  C20  IOSTANDARD LVCMOS18} [get_ports spi_miso]

set_property  -dict {PACKAGE_PIN  N17  IOSTANDARD LVCMOS18} [get_ports clk_out]

create_clock -name rx_clk -period  16.27 [get_ports rx_clk_in]

# probably gone in 2016.4

create_clock -name clk_fpga_0 -period 10 [get_pins "i_system_wrapper/system_i/sys_ps7/inst/PS7_i/FCLKCLK[0]"]
create_clock -name clk_fpga_1 -period  5 [get_pins "i_system_wrapper/system_i/sys_ps7/inst/PS7_i/FCLKCLK[1]"]

create_clock -name spi0_clk      -period 40   [get_pins -hier */EMIOSPI0SCLKO]

set_input_jitter clk_fpga_0 0.3
set_input_jitter clk_fpga_1 0.15

set_property IOSTANDARD LVCMOS18 [get_ports *fixed_io_mio*]
set_property SLEW SLOW [get_ports *fixed_io_mio*]
set_property DRIVE 8 [get_ports *fixed_io_mio*]
set_property  -dict {PACKAGE_PIN E6   PULLTYPE PULLUP} [get_ports fixed_io_mio[ 0]]
set_property  -dict {PACKAGE_PIN A7   PULLTYPE PULLUP} [get_ports fixed_io_mio[ 1]]
set_property  -dict {PACKAGE_PIN B8                  } [get_ports fixed_io_mio[ 2]]
set_property  -dict {PACKAGE_PIN D6                  } [get_ports fixed_io_mio[ 3]]
set_property  -dict {PACKAGE_PIN B7                  } [get_ports fixed_io_mio[ 4]]
set_property  -dict {PACKAGE_PIN A6                  } [get_ports fixed_io_mio[ 5]]
set_property  -dict {PACKAGE_PIN A5                 } [get_ports fixed_io_mio[ 6]]
set_property  -dict {PACKAGE_PIN D8                  } [get_ports fixed_io_mio[ 7]]
set_property  -dict {PACKAGE_PIN D5                  } [get_ports fixed_io_mio[ 8]]
set_property  -dict {PACKAGE_PIN B5   PULLTYPE PULLUP} [get_ports fixed_io_mio[ 9]]
set_property  -dict {PACKAGE_PIN E9   PULLTYPE PULLUP} [get_ports fixed_io_mio[10]]
set_property  -dict {PACKAGE_PIN C6  PULLTYPE PULLUP} [get_ports fixed_io_mio[11]]
set_property  -dict {PACKAGE_PIN D9   PULLTYPE PULLUP} [get_ports fixed_io_mio[12]]
set_property  -dict {PACKAGE_PIN E8   PULLTYPE PULLUP} [get_ports fixed_io_mio[13]]
set_property  -dict {PACKAGE_PIN C5   PULLTYPE PULLUP} [get_ports fixed_io_mio[14]]
set_property  -dict {PACKAGE_PIN C8  PULLTYPE PULLUP} [get_ports fixed_io_mio[15]]
set_property  -dict {PACKAGE_PIN C16  PULLTYPE PULLUP} [get_ports fixed_io_mio[16]]
set_property  -dict {PACKAGE_PIN C13  PULLTYPE PULLUP} [get_ports fixed_io_mio[17]]
set_property  -dict {PACKAGE_PIN C15  PULLTYPE PULLUP} [get_ports fixed_io_mio[18]]
set_property  -dict {PACKAGE_PIN E16  PULLTYPE PULLUP} [get_ports fixed_io_mio[19]]
set_property  -dict {PACKAGE_PIN A14  PULLTYPE PULLUP} [get_ports fixed_io_mio[20]]
set_property  -dict {PACKAGE_PIN D15  PULLTYPE PULLUP} [get_ports fixed_io_mio[21]]
set_property  -dict {PACKAGE_PIN A12  PULLTYPE PULLUP} [get_ports fixed_io_mio[22]]
set_property  -dict {PACKAGE_PIN F12  PULLTYPE PULLUP} [get_ports fixed_io_mio[23]]
set_property  -dict {PACKAGE_PIN A11  PULLTYPE PULLUP} [get_ports fixed_io_mio[24]]
set_property  -dict {PACKAGE_PIN A10  PULLTYPE PULLUP} [get_ports fixed_io_mio[25]]
set_property  -dict {PACKAGE_PIN E13  PULLTYPE PULLUP} [get_ports fixed_io_mio[26]]
set_property  -dict {PACKAGE_PIN C18  PULLTYPE PULLUP} [get_ports fixed_io_mio[27]]
set_property  -dict {PACKAGE_PIN B12  PULLTYPE PULLUP} [get_ports fixed_io_mio[28]]
set_property  -dict {PACKAGE_PIN C12  PULLTYPE PULLUP} [get_ports fixed_io_mio[29]]
set_property  -dict {PACKAGE_PIN C10  PULLTYPE PULLUP} [get_ports fixed_io_mio[30]]
set_property  -dict {PACKAGE_PIN C11  PULLTYPE PULLUP} [get_ports fixed_io_mio[31]]

set_property IOSTANDARD LVCMOS18 [get_ports *fixed_io_ps*]
set_property SLEW SLOW [get_ports *fixed_io_ps*]
set_property DRIVE 8 [get_ports *fixed_io_ps*]
set_property PACKAGE_PIN E7 [get_ports fixed_io_ps_clk]
set_property PACKAGE_PIN C7 [get_ports fixed_io_ps_porb]

set_property IOSTANDARD SSTL15_T_DCI [get_ports *fixed_io_ddr_vr*]
set_property SLEW FAST [get_ports *fixed_io_ddr_vr*]
set_property PACKAGE_PIN H5 [get_ports fixed_io_ddr_vrp]
set_property PACKAGE_PIN G5 [get_ports fixed_io_ddr_vrn]

set_property IOSTANDARD DIFF_SSTL15 [get_ports *ddr_ck*]
set_property SLEW FAST [get_ports *ddr_ck*]
set_property PACKAGE_PIN L2 [get_ports ddr_ck_p]
set_property PACKAGE_PIN M2 [get_ports ddr_ck_n]

set_property IOSTANDARD SSTL15 [get_ports *ddr_addr*]
set_property SLEW SLOW [get_ports *ddr_addr*]
set_property PACKAGE_PIN N2 [get_ports ddr_addr[0]]
set_property PACKAGE_PIN K2 [get_ports ddr_addr[1]]
set_property PACKAGE_PIN M3 [get_ports ddr_addr[2]]
set_property PACKAGE_PIN K3 [get_ports ddr_addr[3]]
set_property PACKAGE_PIN M4 [get_ports ddr_addr[4]]
set_property PACKAGE_PIN L1 [get_ports ddr_addr[5]]
set_property PACKAGE_PIN L4 [get_ports ddr_addr[6]]
set_property PACKAGE_PIN K4 [get_ports ddr_addr[7]]
set_property PACKAGE_PIN K1 [get_ports ddr_addr[8]]
set_property PACKAGE_PIN J4 [get_ports ddr_addr[9]]
set_property PACKAGE_PIN F5 [get_ports ddr_addr[10]]
set_property PACKAGE_PIN G4 [get_ports ddr_addr[11]]
set_property PACKAGE_PIN E4 [get_ports ddr_addr[12]]
set_property PACKAGE_PIN D4 [get_ports ddr_addr[13]]
set_property PACKAGE_PIN F4 [get_ports ddr_addr[14]]

set_property IOSTANDARD SSTL15 [get_ports *ddr_ba*]
set_property SLEW SLOW [get_ports *ddr_ba*]
set_property PACKAGE_PIN L5 [get_ports ddr_ba[0]]
set_property PACKAGE_PIN R4 [get_ports ddr_ba[1]]
set_property PACKAGE_PIN J5 [get_ports ddr_ba[2]]

set_property IOSTANDARD SSTL15 [get_ports ddr_reset_n]
set_property SLEW FAST [get_ports ddr_reset_n]
set_property PACKAGE_PIN B4 [get_ports ddr_reset_n]
set_property IOSTANDARD SSTL15 [get_ports ddr_cs_n]
set_property SLEW SLOW [get_ports ddr_cs_n]
set_property PACKAGE_PIN N1 [get_ports ddr_cs_n]
set_property IOSTANDARD SSTL15 [get_ports ddr_ras_n]
set_property SLEW SLOW [get_ports ddr_ras_n]
set_property PACKAGE_PIN P4 [get_ports ddr_ras_n]
set_property IOSTANDARD SSTL15 [get_ports ddr_cas_n]
set_property SLEW SLOW [get_ports ddr_cas_n]
set_property PACKAGE_PIN P5 [get_ports ddr_cas_n]
set_property IOSTANDARD SSTL15 [get_ports ddr_we_n]
set_property SLEW SLOW [get_ports ddr_we_n]
set_property PACKAGE_PIN M5 [get_ports ddr_we_n]
set_property IOSTANDARD SSTL15 [get_ports ddr_cke]
set_property SLEW SLOW [get_ports ddr_cke]
set_property PACKAGE_PIN N3 [get_ports ddr_cke]
set_property IOSTANDARD SSTL15 [get_ports ddr_odt]
set_property SLEW SLOW [get_ports ddr_odt]
set_property PACKAGE_PIN N5 [get_ports ddr_odt]

set_property IOSTANDARD SSTL15_T_DCI [get_ports *ddr_dq[*]]
set_property SLEW FAST [get_ports *ddr_dq[*]]
set_property PACKAGE_PIN C3 [get_ports ddr_dq[0]]
set_property PACKAGE_PIN B3 [get_ports ddr_dq[1]]
set_property PACKAGE_PIN A2 [get_ports ddr_dq[2]]
set_property PACKAGE_PIN A4 [get_ports ddr_dq[3]]
set_property PACKAGE_PIN D3 [get_ports ddr_dq[4]]
set_property PACKAGE_PIN D1 [get_ports ddr_dq[5]]
set_property PACKAGE_PIN C1 [get_ports ddr_dq[6]]
set_property PACKAGE_PIN E1 [get_ports ddr_dq[7]]
set_property PACKAGE_PIN E2 [get_ports ddr_dq[8]]
set_property PACKAGE_PIN E3 [get_ports ddr_dq[9]]
set_property PACKAGE_PIN G3 [get_ports ddr_dq[10]]
set_property PACKAGE_PIN H3 [get_ports ddr_dq[11]]
set_property PACKAGE_PIN J3 [get_ports ddr_dq[12]]
set_property PACKAGE_PIN H2 [get_ports ddr_dq[13]]
set_property PACKAGE_PIN H1 [get_ports ddr_dq[14]]
set_property PACKAGE_PIN J1 [get_ports ddr_dq[15]]
set_property IOSTANDARD SSTL15_T_DCI [get_ports *ddr_dm[*]]
set_property SLEW FAST [get_ports *ddr_dm[*]]
set_property PACKAGE_PIN A1 [get_ports ddr_dm[0]]
set_property PACKAGE_PIN F1 [get_ports ddr_dm[1]]
set_property IOSTANDARD DIFF_SSTL15_T_DCI [get_ports *ddr_dqs*]
set_property SLEW FAST [get_ports *ddr_dqs*]
set_property PACKAGE_PIN C2 [get_ports ddr_dqs_p[0]]
set_property PACKAGE_PIN B2 [get_ports ddr_dqs_n[0]]
set_property PACKAGE_PIN G2 [get_ports ddr_dqs_p[1]]
set_property PACKAGE_PIN F2 [get_ports ddr_dqs_n[1]]

set_false_path -from [get_pins {i_system_wrapper/system_i/axi_ad9361/inst/i_rx/i_up_adc_common/up_adc_gpio_out_int_reg[0]/C}]
set_false_path -from [get_pins {i_system_wrapper/system_i/axi_ad9361/inst/i_tx/i_up_dac_common/up_dac_gpio_out_int_reg[0]/C}]

