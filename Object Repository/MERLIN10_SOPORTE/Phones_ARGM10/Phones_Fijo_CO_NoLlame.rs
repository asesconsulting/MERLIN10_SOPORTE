<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Phones_Fijo_CO_NoLlame</name>
   <tag></tag>
   <elementGuidId>7ed94013-9b61-4217-8d39-cd5c0056927e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.phoneonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:phoneNormalize2>
         &lt;!--Optional:-->
         &lt;phoneNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xPhone>
               &lt;!--Optional:-->
               &lt;level1> &lt;/level1>
               &lt;!--Optional:-->
               &lt;level2> &lt;/level2>
               &lt;!--Optional:-->
               &lt;level3> &lt;/level3>
               &lt;!--Optional:-->
               &lt;level4> &lt;/level4>
               &lt;!--Optional:-->
               &lt;level5> &lt;/level5>
               &lt;!--Optional:-->
               &lt;characteristic> &lt;/characteristic>
               &lt;!--Optional:-->
               &lt;phoneNumber>1149233521&lt;/phoneNumber>
               &lt;!--Optional:-->
               &lt;postalCode> &lt;/postalCode>
               &lt;!--Optional:-->
               &lt;ddi> &lt;/ddi>
               &lt;!--Optional:-->
               &lt;ddn> &lt;/ddn>
               &lt;!--Optional:-->
               &lt;additionalData> &lt;/additionalData>
            &lt;/xPhone>
         &lt;/phoneNormalize>
      &lt;/soap:phoneNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>phoneNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Phone_ARG2</defaultValue>
      <description></description>
      <id>0a2a47f4-63f3-45e2-80ee-cf9d02408ebc</id>
      <masked>false</masked>
      <name>Phone_ARG2</name>
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


WS.verifyElementText(response, 'phoneNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'phoneNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.level1', 'AR')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.level3', 'TRES DE FEBRERO')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.level4', '11 DE SEPTIEMBRE')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.level5', '')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.postalCode', '0')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.ddi', '')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.ddn', '011')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.characteristic', '4923')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.phoneNumber', '3521')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.validated', 'NO')
WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.additionalData', '')




WS.verifyElementText(response, 'phoneNormalize2Response.return.nPhone.merlinCustomValues.merlinCustomValues[0].value', '01149233521')




</verificationScript>
   <wsdlAddress>${Phone_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
