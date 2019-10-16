<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Filiation_VA_OK _TributaryType_Diference</name>
   <tag></tag>
   <elementGuidId>c6849256-835c-4cc3-bba6-809d50e4cc35</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.filiationonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:filiationNormalize2>
         &lt;!--Optional:-->
         &lt;filiationNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>7d6bb345ce19a5c55627b8cb1f7f506b&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xFiliation>
               &lt;!--Optional:-->
               &lt;documentType> &lt;/documentType>
               &lt;!--Optional:-->
               &lt;documentNumber>&lt;/documentNumber>
               &lt;!--Optional:-->
               &lt;tributaryType>80&lt;/tributaryType>
               &lt;!--Optional:-->
               &lt;tributaryNumber>27062686613&lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName> &lt;/lastName>
               &lt;!--Optional:-->
               &lt;name> &lt;/name>
               &lt;!--Optional:-->
               &lt;gender> &lt;/gender>
               &lt;!--Optional:-->
               &lt;birthDate> &lt;/birthDate>
            &lt;/xFiliation>
         &lt;/filiationNormalize>
      &lt;/soap:filiationNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>filiationNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Filiator_ARG2</defaultValue>
      <description></description>
      <id>3cbbeb4b-0c9e-4ae0-96a1-7c2afb91f051</id>
      <masked>false</masked>
      <name>Filiator_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementText(response, 'filiationNormalize2Response.return.status', 'OK')
WS.verifyElementText(response, 'filiationNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentType', '96')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentTypeFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentNumber', '6268661')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentNumberFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryType', '86')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryTypeFlg', 'CO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryNumber', '27062686613')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryNumberFlg', 'VA')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.contributorType', 'A')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.contributorTypeFlg', 'CO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.lastName', 'KUZNICKI')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.lastNameFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.name', 'MARTA ALEJANDRA')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.nameFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.gender', 'F')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.genderFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.birthDate', '05/04/1949')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.birthDateFlg', 'AP')
assertThat(response.getResponseText()).contains('&lt;name>differenceLevelName&lt;/name>&lt;value>1&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>usuario&lt;/name>&lt;value>MERLIN&lt;/value>')</verificationScript>
   <wsdlAddress>${Filiator_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
