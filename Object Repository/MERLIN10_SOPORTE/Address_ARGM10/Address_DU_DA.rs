<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DA</name>
   <tag></tag>
   <elementGuidId>196d7884-f9be-4dab-a897-0e61f3a7529c</elementGuidId>
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
               &lt;level2>&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>Villa Ballester&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>SAN JOSE DE FLORES 6001&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber> &lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>1653&lt;/postalCode>
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
      <id>570e04ea-3e46-4de0-baed-f16aed15b09c</id>
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
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'DA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'VILLA BALLESTER')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'SAN JOSE DE FLORES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '6001')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1653')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '6')


WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].geoType', '8')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].level3', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].level4', 'VILLA GODOY CRUZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].street', 'SAN JOSE DE FLORES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].houseNumber', '6001')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].latitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].longitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].fromStreetNumber', '6201')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.alternativeAddresses[0].toStreetNumber', '6900')</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
