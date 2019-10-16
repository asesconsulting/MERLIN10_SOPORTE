<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Filiation_VA_OK _DocumentType_Diference</name>
   <tag></tag>
   <elementGuidId>d1ac8321-58fa-4b5d-87d1-0876dedb63f7</elementGuidId>
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
               &lt;documentType>90&lt;/documentType>
               &lt;!--Optional:-->
               &lt;documentNumber>25471677&lt;/documentNumber>
               &lt;!--Optional:-->
               &lt;tributaryType> &lt;/tributaryType>
               &lt;!--Optional:-->
               &lt;tributaryNumber>&lt;/tributaryNumber>
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
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentTypeFlg', 'CO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentNumber', '25471677')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentNumberFlg', 'VA')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryType', '80')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryTypeFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryNumber', '20254716775')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryNumberFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.contributorType', 'A')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.contributorTypeFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.lastName', 'FERRERO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.lastNameFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.name', 'PABLO NORBERTO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.nameFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.gender', 'M')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.genderFlg', 'AP')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.birthDate', '11/11/1976')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.birthDateFlg', 'AP')
assertThat(response.getResponseText()).contains('&lt;name>differenceLevelName&lt;/name>&lt;value>1&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>usuario&lt;/name>&lt;value>MERLIN&lt;/value>')</verificationScript>
   <wsdlAddress>${Filiator_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
