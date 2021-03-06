import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation2.1_DudaMultiple_CambioRespuesta'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation2.1_DudaMultiple_SANCOR'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_DU'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_DU_Bertone_Maria'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_DU_Mapfre_Despejada'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_DU_Nombre'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_NE'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_OK_Maria_Ines'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_VA_Forma_Juridica_Sancor'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_VA_Hsbc_Tag_Score'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_VA_Macro'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_VA_OK'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_VA_OK _DocumentType_Diference'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_VA_OK _TributaryType_Diference'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_VA_OK_Empresa'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Filiation_ARGM10/Filiation_VA_OK_Nombre'))

