<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DS</name>
   <tag></tag>
   <elementGuidId>a497d77f-5613-4738-897f-53ddc9e4c7c8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.addressonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:addressNormalize2>
         &lt;!--Optional:-->
         &lt;addressNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter>&lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xAddress>
               &lt;!--Optional:-->
               &lt;level1>&lt;/level1>
               &lt;!--Optional:-->
               &lt;level2>SALTA&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>SALTA&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>GUEMES&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber>1768&lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>4400&lt;/postalCode>
               &lt;!--Optional:-->
               &lt;additionalData>&lt;/additionalData>
               &lt;!--Optional:-->
               &lt;betweenStreet1>&lt;/betweenStreet1>
               &lt;!--Optional:-->
               &lt;betweenStreet2>&lt;/betweenStreet2>
            &lt;/xAddress>
         &lt;/addressNormalize>
      &lt;/soap:addressNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>addressNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Address_ARG2</defaultValue>
      <description></description>
      <id>64ed9e66-32c2-45d4-9add-e1dcb7c2190b</id>
      <masked>false</masked>
      <name>Address_ARG2</name>
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


WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'DU')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'DE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'SALTA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'SALTA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'GUEMES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '1768')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '4400')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '2')



WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].geoType', '8')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].level2', 'SALTA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].level3', 'CAPITAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].level4', 'SALTA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].level5', 'SAN MARTIN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].street', 'GRAL MARTIN MIGUEL DE GUEMES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].houseNumber', '1768')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].latitude', '-24.785015')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].longitude', '-65.426701')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].postalCode', '4400')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].fromStreetNumber', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].toStreetNumber', '2400')</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
